---
name: animation_performance_and_accessibility.md
description: Use when the agent is adding motion to a UI (transitions, micro-interactions, hover/focus effects, page transitions, drag feedback, loading spinners, skeleton screens, progress indicators, parallax, carousels, reveal-on-scroll); choosing which CSS properties to animate; deciding timing, duration, or easing curves; using will-change, transform, opacity, or the compositor; building loading or empty states; handling animation interruption and mid-animation state changes; respecting prefers-reduced-motion and motion sensitivity; deciding whether an animation helps or harms the experience; or reviewing a UI for jank, dropped frames, motion sickness, or unnecessary motion. Also covers the tension between expressive motion and accessibility, the difference between decorative and functional animation, and the failure modes of animating layout-triggering properties, over-animating, and shipping motion that some users cannot safely perceive.
---

# Animation Performance And Accessibility

Animation is a judgment about *what should change over time, why, and at what cost.* Every animation makes three claims at once: it claims the user's attention, it claims CPU/GPU/energy budget, and it claims that the change it depicts is worth perceiving. When those claims are wrong, the cost is concrete: a janky frame budget that makes the interface feel broken, a battery-draining effect on a mobile device, a loading animation that lies about progress, or motion that triggers vestibular discomfort, nausea, or seizures in a subset of users. Animation is never neutral decoration — it is an attention and accessibility decision that the implementer is making whether or not they realize it.

Agents tend to under-invest in this judgment for two opposite reasons. The first is treating animation as polish added at the end: a fade here, a slide there, a "nice to have" that cannot hurt. The second is treating animation as impressive: long, elaborate transitions that look great in a demo and ship poorly. Both miss the point. The point is that animation has a *purpose* — to guide attention, confirm an action, communicate a state change, or reduce perceived latency — and when that purpose is unclear or the implementation is expensive, the animation actively degrades the experience. The judgment problem is deciding, for each piece of motion, whether it earns its place, which property it should run on, how fast it should move, and how it behaves for the user who cannot or should not perceive it.

This skill covers animation purpose, the performance properties (transform/opacity vs layout-triggering properties, compositor offloading, will-change), timing and easing, micro-interaction design, loading and skeleton states, interruption and mid-animation changes, and the resolution of the motion-vs-accessibility tension. It complements the accessibility skill, which owns the broader WCAG and assistive-technology surface; here the question is motion specifically.

## Core Rules

### Decide Whether Each Animation Has A Purpose Before Adding It

Motion is justified when it does one of a small number of jobs, and unjustified otherwise. Before animating, name which job it performs:

- **Spatial or structural guidance.** Showing *where* something came from or went (a panel sliding in from the side it was anchored to, an item moving to a cart). This helps the user build a mental model of the interface's structure.
- **Action confirmation and feedback.** Showing that an input was received (a button press, a toggle flipping, a checkmark appearing). This closes the loop between intent and result.
- **State-change communication.** Showing a transition between states so the user understands what changed (a collapse, an expand, a reorder) rather than experiencing an abrupt jump they must re-parse.
- **Perceived-latency reduction.** A skeleton screen or progress indicator that makes a wait feel shorter or at least honest, by showing that work is happening.

Motion that does none of these — a hero that flies in because it looks cool, text that fades in word by word, an icon that spins perpetually — is decorative. Decorative motion is not forbidden, but it carries cost (attention, performance, accessibility) and must clear a higher bar. Ask, for each animation: if I removed it, would the user be confused, slower, or less confident? If the answer is no, the animation is probably not earning its place. When in doubt, ship less motion; you can always add it once the need is real.

### Animate Compositor-Friendly Properties, Not Layout-Triggering Ones

The single most important performance rule: animate `transform` and `opacity` (and properties the browser can composite without recalculating layout), and avoid animating properties that force layout or paint on every frame. The browser's rendering pipeline is roughly style → layout → paint → composite. Animating a layout-triggering property (`width`, `height`, `top`, `left`, `margin`, `padding`, `font-size`) forces the browser to redo layout for the whole affected subtree on every frame; animating a paint-triggering property (`color`, `background`, `box-shadow`) forces a repaint; only `transform` and `opacity` can be handled by the compositor alone, on a separate thread, without touching layout or paint.

Concretely:

- **To move something, animate `transform: translate()`**, not `top`/`left`/`margin`. Moving via layout properties reflows the page each frame.
- **To resize, animate `transform: scale()`** when the visual result is acceptable, not `width`/`height`. Scaling is composited; resizing layout is not.
- **To rotate, animate `transform: rotate()`.** To fade, animate `opacity`.** Both are cheap.
- **`box-shadow` animation is expensive** because it repaints; prefer animating a pre-rendered shadow on a pseudo-element's `opacity`, or accept the cost knowingly.

This is not a stylistic preference; it is the difference between a 60fps animation and a janky one. The exception is when a layout animation is genuinely required (animating to `auto` height for a collapsible). In that case, measure the target height with the compositor-friendly `scale`/`clip` techniques, or accept that the animation will be more expensive and keep it short.

### Keep Animations Short, And Match Duration To The Job

Animation duration is a usability decision, not an aesthetic one. The well-established ranges, grounded in human perception:

- **Micro-interactions and state feedback: 100–200ms.** A button press, a toggle, a hover state. Fast enough to feel immediate and not delay the user's next action. Below ~100ms the change feels abrupt (a jump); above ~200ms it starts to feel sluggish for a simple confirmation.
- **Transitions and reveals: 200–400ms.** A panel sliding in, a modal appearing, a list reordering. Long enough to be perceived as motion rather than a jump, short enough not to test patience.
- **Larger structural changes: up to ~500ms, rarely more.** A page transition or a major view swap. Anything longer than about half a second makes the user wait for the interface rather than use it.

The trap is the "demo effect": animations that look impressive at five seconds in a portfolio but feel slow at the hundredth use. Optimize for the user who performs the action repeatedly, not the viewer seeing it once. When an animation must be longer (a progress indicator tied to real work), make it honest about the work rather than a fixed-duration delay.

### Choose Easing That Reflects Physicality, Not Linear Motion

Almost nothing in a well-designed interface moves at a constant (linear) rate. Linear motion looks mechanical and dead because real objects accelerate and decelerate. The defaults that read as natural:

- **Ease-out (fast start, slow end) for elements entering or growing.** The element arrives and settles, which feels responsive and deliberate. This is the default for most UI motion.
- **Ease-in (slow start, fast end) for elements leaving or shrinking.** The element accelerates away. Use sparingly; pure ease-in can feel like the element is falling off the screen.
- **Ease-in-out for movement across a space** (an element traveling a distance), where both acceleration and deceleration read as physical.
- **Spring/physics-based easing for interactive elements** that should feel like they respond to the user's input (drag, fling, overshoot). Springs feel alive but require tuning; a poorly tuned spring overshoots nauseatingly.

Avoid linear easing for anything organic (it reads as a loading bar, not a UI element). Avoid aggressive `cubic-bezier` curves with overshoot for functional motion unless the overshoot communicates something (a pull-to-refresh, a snap). Match the easing to the *feeling* the interaction should have: confident motion eases out quickly; playful motion can overshoot; urgent motion is short and direct.

### Design Micro-Interactions As A Closed Feedback Loop

A micro-interaction is a small, self-contained animation tied to a single user action (a like button, a toggle, a drag, a swipe). The judgment is whether the interaction communicates the full loop: trigger → change → result. A weak micro-interaction animates only the visual; a strong one communicates *what just happened and what state I am now in*.

For each micro-interaction, ensure:

- **The trigger is acknowledged immediately.** The moment of input (press, tap, hover) produces a visible response within a frame; any perceptible delay feels broken.
- **The state change is legible.** A toggle that animates should make its new on/off state unambiguous by the end, not merely "move."
- **The end state is stable and reachable.** The animation must always settle into a defined end state, even if interrupted (see the next rule). A toggle stuck mid-flip is a defect.
- **The motion reinforces meaning, not fights it.** A "delete" action should not bounce playfully; a "success" should not shake. The motion's character should match the semantic weight of the action.

Micro-interactions are where over-animation is most tempting and most damaging. A button that bounces, glows, and ripples on every press becomes noise; reserve expressive motion for moments that warrant emphasis, and keep routine feedback calm and quick.

### Make Animations Interruptible And Never Lock The User Out

A common failure is an animation that cannot be interrupted: the user clicks a button, a 400ms transition begins, and the user's next input is ignored or queued until the animation finishes. On a fast device this is invisible; on a slow device or a repeated action, it makes the interface feel unresponsive. The rule: **the user's intent should always take precedence over in-flight motion.**

- **New input should cancel or redirect a running animation, not wait for it.** If a user toggles a panel open and immediately toggles it closed, the close should begin from wherever the open animation currently is, not from the fully-open state.
- **Animate from the current rendered state, not from a stored start.** Read the current value (via the Web Animations API, FLIP technique, or framework primitives) so an interruption picks up mid-flight smoothly.
- **Avoid disabling interaction during animation** unless there is a genuine reason (preventing a double-submit). A modal that blocks input for its entire entrance animation feels slow; let the user act as soon as the content is usable.
- **Loading animations must not lie about progress.** A progress bar that fills to 90% and sticks, or a spinner that runs forever with no end, erodes trust. Either show real progress, or use an indeterminate indicator honestly labeled as such.

The test: perform the action rapidly and repeatedly, and on a throttled CPU. If the interface queues inputs or jumps to wrong states, the animation is not interruptible.

### Use Skeletons And Honest Loading States, Not Infinite Spinners

Loading state is where animation most often fails the user by being dishonest. A blank screen followed by a sudden pop-in feels broken; a spinner that spins indefinitely with no signal of progress feels hopeless. The strong patterns:

- **Skeleton screens** (a wireframe of the upcoming content) reduce perceived latency because they show *shape* immediately and let the user anticipate the content. They should match the final layout closely so the real content does not cause a jarring reflow.
- **Progressive loading** of content as it arrives (images fading in, text appearing) feels faster than waiting for everything, as long as the layout is reserved to avoid shift.
- **Determinant progress** when the work is measurable (a real percentage, steps completed) is always preferable to indeterminate motion when you can compute it.
- **Honest indeterminate states** when progress is unknown: a clearly labeled "Loading…" with motion that signals activity without implying a near end. Avoid the 99%-stuck progress bar, which is worse than no progress bar.

A loading animation that runs for a second is helpful; one that runs for ten seconds with no change is a broken promise. Tie the animation to the actual state of the work, and provide an escape (retry, cancel, informative error) when the work fails.

### Respect Reduced Motion As A First-Class Requirement, Not A Fallback

A meaningful fraction of users experience motion negatively: vestibular disorders, motion sensitivity, migraines, and (at the extreme) photo-induced seizures. For these users, parallax, large-scale movement, autoplaying motion, and rapid flashing are not annoying — they are physically harmful. WCAG 2.1 SC 2.3.3 (Animation from Interactions) and the broader principle require that motion be avoidable. This is not optional polish.

- **Honor `prefers-reduced-motion`.** Provide a reduced-motion path that either removes non-essential animation entirely or replaces it with an instant state change (a fade becomes a cut; a slide becomes an appearance). Test by enabling the OS setting and confirming the experience degrades gracefully.
- **Distinguish essential from non-essential motion.** Essential motion (a progress bar that communicates real work, a loading spinner with no alternative) may remain but should be minimized; non-essential motion (decorative parallax, entrance flourishes, ambient loops) should be removed entirely under reduced motion.
- **Never autoplay large-scale or parallax motion.** Full-screen movement, background parallax tied to scroll, and zooming hero images are the highest-risk motion for vestibular harm. Make them opt-in or omit them.
- **Never use flashing above the seizure threshold.** More than three flashes per second, or high-contrast large-area flashing, is a seizure risk (WCAG 2.3.1). This is a hard line, not a guideline.
- **Do not make motion the only way to perceive something.** A change conveyed only by animation (an item "shaking" to indicate an error, with no text or icon) is invisible to users who disable motion and to screen readers. Always pair motion with a non-motion channel.

The reduced-motion path is not a degraded version of the "real" experience; it is the correct experience for those users. Build it deliberately, not as an afterthought, and verify it actually works rather than assuming the media query is wired.

### Resolve The Motion-Versus-Accessibility Tension Deliberately

Motion and accessibility sometimes pull in opposite directions: an animation that guides a sighted user can disorient a screen-reader user or harm a motion-sensitive one. The resolution is not to pick a side but to make the motion *additive* — useful to those who benefit, invisible or replaced for those who do not.

- **Animation should reinforce information available elsewhere, not be the sole carrier of it.** A toast that slides in should also be announced (live region); an error that shakes should also show text. The sighted user gets the reinforcement; the assistive-technology user gets the content.
- **Reserve expressive motion for low-stakes moments.** A celebratory animation on a game win is fine; the same animation on a medical form submission is not. Match the motion's intensity to the stakes and to who might be harmed.
- **When motion and clarity conflict, clarity wins.** If an animation makes a state change harder to perceive (too slow, too subtle, or masking the new state), it is failing its job. The function is communication; the motion is the medium.

## Common Traps

### Animating Width, Height, Top, Or Left Because It Looks Simple In CSS

Setting `transition: width 0.3s` feels natural in CSS but forces a layout reflow on every frame, producing jank on anything but the fastest device. Animate `transform: scaleX()` or restructure so the compositor handles it; reserve layout-property animation for cases where no compositor-friendly equivalent exists, and keep those short.

### The Demo-Effect Long Animation

A 1.2-second page transition or a word-by-word text reveal that looks stunning in a portfolio and feels agonizing at the hundredth use. Optimize for the repeat user; if an animation would annoy someone who performs the action fifty times a day, it is too long or too elaborate.

### Linear Easing For Organic Motion

Using `linear` timing for a button or panel because it was the default. Linear motion reads as mechanical and dead; almost all UI motion should ease out (entering) or ease in-out (traveling). Reach for linear only for genuinely mechanical things like a progress bar tied to uniform work.

### Overusing will-change As A "Performance Fix"

Sprinkling `will-change: transform` on many elements to "make them faster." `will-change` hints the browser to promote the element to its own layer and reserve resources; overusing it consumes GPU memory and can *hurt* performance by forcing too many layers. Apply it narrowly to elements about to animate, and remove it when the animation is done, rather than leaving it as a permanent style.

### The 99%-Stuck Progress Bar

A progress bar that fills to 90% quickly and then creeps or stalls, because the underlying work is indeterminate but the designer wanted it to "look like progress." This is worse than an honest indeterminate spinner because it implies a near end that never arrives. Use real progress when measurable; use a clearly indeterminate indicator when not.

### Non-Interruptible Animations That Queue Inputs

A toggle or panel whose animation must complete before the next interaction registers, so a user who clicks twice rapidly sees the animation play out fully twice, or worse, ends in the wrong state. Read the current state on each input and redirect the animation; never queue user intent behind motion.

### Decorative Parallax Or Autoplay Motion With No Reduced-Motion Path and motion As The Only Signal

A hero with scroll-tied parallax and ambient floating elements, shipped without a `prefers-reduced-motion` alternative. For motion-sensitive users this is not a minor annoyance — it can trigger vestibular discomfort or nausea. Build the reduced-motion path as part of the feature, not as a bug fix after complaints.

An error indicated only by a shake animation, or a "saved" state shown only by a brief glow. Users with reduced motion, screen readers, or who simply weren't looking miss the signal entirely. Pair every motion-conveyed state with text, an icon, or an announcement.

### Flashing Or High-Contrast Motion Near The Seizure Threshold and skeleton Screens That Don't Match The Final Layout

A loading animation, error indicator, or video overlay that flashes rapidly or alternates high-contrast colors more than three times per second. This is a hard accessibility failure (WCAG 2.3.1) and a real seizure trigger, not a style preference. Keep any flashing well below the threshold, and avoid large-area high-contrast flashing entirely.

A skeleton placeholder whose shape differs from the real content, so when the content arrives the layout jumps (layout shift) and the skeleton-to-content transition is jarring. A skeleton should reserve the real layout's dimensions; otherwise it creates the very jank it was meant to prevent.

## Self-Check

- [ ] Each animation has a named purpose (spatial guidance, action feedback, state-change communication, or perceived-latency reduction); decorative motion was challenged and kept only where it clears a higher bar, not added because it "looks nice."
- [ ] Animations run on compositor-friendly properties (`transform`, `opacity`) rather than layout-triggering ones (`width`, `height`, `top`, `left`, `margin`, `padding`); any layout-property animation is short and knowingly accepted.
- [ ] Durations match the job (micro-interactions ~100–200ms, transitions ~200–400ms, larger changes ≤~500ms), optimized for the repeat user rather than the first-time viewer, with no demo-effect long animations.
- [ ] Easing reflects physicality (ease-out for entering, ease-in-out for traveling, springs only where tuned and meaningful); linear easing is used only for genuinely mechanical motion, not organic UI.
- [ ] Micro-interactions form a closed feedback loop: immediate trigger acknowledgement, legible end state, stable settlement, and motion character that matches the action's semantic weight.
- [ ] Animations are interruptible: new input cancels or redirects in-flight motion from the current rendered state, interaction is not blocked during animation, and rapid repeated actions do not queue or land in wrong states (verified on a throttled CPU).
- [ ] Loading states are honest: skeletons match the final layout to avoid shift, progressive loading reserves space, determinant progress is used when measurable, and no 99%-stuck progress bars are shipped.
- [ ] `prefers-reduced-motion` is honored with a deliberate reduced-motion path (non-essential motion removed, essential motion minimized), no autoplay parallax or large-scale motion, no flashing above the seizure threshold (≤3 flashes/sec), and motion is never the sole carrier of information.
- [ ] Motion is additive to accessibility: every motion-conveyed state is paired with text, an icon, or a live-region announcement, so users who disable motion or use assistive technology perceive the same information.
- [ ] The highest-risk cases were verified — layout-property jank on a slow device, non-interruptible input queuing, the reduced-motion path actually engaging, and any flashing against the seizure threshold — not only the smooth 60fps path on a fast machine.
