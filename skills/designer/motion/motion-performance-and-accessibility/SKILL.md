---
name: motion_performance_and_accessibility.md
description: Use when the agent is specifying motion that must remain performant and accessible, including animation on low-end devices, reduced-motion preferences, vestibular safety, autoplay and looping motion, parallax, scroll-driven animation, and the tradeoffs between visual richness and inclusivity.
---

# Motion Performance And Accessibility

Motion that looks beautiful on a designer's high-end machine can stutter on a low-end phone, drain a laptop battery, or make a user physically ill. Performance and accessibility are not afterthoughts to motion design; they are constraints that shape which motions are acceptable at all. The judgment problem is that motion is specified in idealized conditions and then degrades unpredictably in the real world, and the designer who does not plan for that degradation ships motion that excludes users, frustrates users, or harms them.

Use this skill before approving any animation, especially looping, autoplay, parallax, scroll-driven, or large-scale motion, and before assuming motion will run smoothly everywhere. The goal is to prevent the agent from specifying motion that performs poorly, ignores accessibility preferences, or creates vestibular risk.

## Core Rules

### Design For The Slowest Plausible Device

Motion is specified on fast hardware and experienced on slow hardware. A 60-frame-per-second animation on a development machine can drop to a stuttering slideshow on a budget phone, an older laptop, or a device under load. Stuttering motion is worse than no motion, because it draws attention to the lag and feels broken.

When specifying motion, assume it will run on mid-range and low-end devices. Favor properties the browser can animate cheaply, such as transform and opacity, and avoid animating layout-triggering properties such as width, height, top, left, or margin, which force the browser to recalculate the page on every frame. Keep animated element counts low, and avoid animating large areas of the screen simultaneously.

### Respect Reduced-Motion Preferences As A Health Requirement

Operating systems expose a reduced-motion preference that users set for a reason: motion causes them discomfort, dizziness, nausea, migraines, or disorientation. This is not a stylistic preference. Vestibular disorders affect a meaningful share of users, and motion that rotates, scales, parallax-scrolls, or moves large elements can trigger symptoms.

Every animated interface must detect the reduced-motion preference and provide an alternative that minimizes or removes movement. Acceptable reduced-motion alternatives include instant state changes, simple crossfades, or motion confined to opacity with no spatial travel. The reduced-motion path must be designed and tested, not assumed to be a simple flag that disables everything.

### Identify And Avoid Vestibular Triggers

Some motion is more dangerous than others. The highest-risk patterns are those that create a false sense of self-motion or large-scale spatial movement: full-screen slides, parallax where background and foreground move at different rates, zooming or scaling of the whole view, horizontal panning, and any motion that continues after the user stops scrolling.

Audit each animation for vestibular risk. If the motion moves the majority of the viewport, simulates camera movement, or persists autonomously, treat it as high-risk and either remove it, constrain it, or gate it behind the reduced-motion preference by default. Small, contained motion near a trigger is usually safe; large, ambient, or continuous motion usually is not.

### Limit Autoplay And Looping Motion

Motion that plays without user initiation competes for attention and can overwhelm users with sensory sensitivity, ADHD, or cognitive fatigue. Autoplay and infinite loops are especially costly because the user cannot control them.

Constrain autoplay and looping motion to contexts where it is justified, such as a brief loading indicator or a clearly optional demo, and avoid it on functional surfaces where the user is trying to read, decide, or act. Where looping motion is necessary, keep it small, subtle, and easy to ignore, and ensure it pauses when off-screen or when the user interacts.

### Prefer Cheap, Composited Properties

Performance is a usability and accessibility issue, because janky motion excludes users on slower devices. Animate transform and opacity, which the browser can composite on the GPU without recalculating layout. Avoid animating properties that trigger layout or paint, such as box-shadow, border-radius, width, height, and position offsets, unless the element is small and isolated.

Keep will-change usage scoped and temporary, applying it only to elements about to animate and removing it after, to avoid memory overhead. The goal is motion that stays smooth across the device range, not just on the device it was built on.

### Make Motion Optional Where Stakes Are Low

For motion that is purely decorative, the default should often be no motion or minimal motion, with richer motion available to users who want it. For motion that is functional, such as a transition that communicates continuity, keep it but ensure it is fast, cheap, and reduced-motion-aware.

The question to ask is whether the motion is earning its cost. Decorative motion that performs poorly or excludes users is not earning its cost. Functional motion that communicates and respects preferences usually is.

### Test Motion Under Real Conditions

Motion that looks correct in a prototype can fail in production because of network timing, re-renders, data loading, or device variability. Test animations on representative low-end hardware, with realistic data, under reduced-motion, and during interruption. Watch for jank, for motion that outlasts the data it accompanies, and for reduced-motion paths that still move too much.

Testing only on the designer's machine is how performance and accessibility failures ship.

### Provide Controls For Continuous Motion

When a surface includes continuous or autoplay motion, give the user a way to pause, stop, or hide it. This is both an accessibility expectation and a respect for user attention. A pause control, an option to reduce motion in-product, or motion that stops when the user scrolls past it all reduce the burden.

Users should never be trapped watching motion they did not ask for.

## Common Traps

### Specifying Motion Only For High-End Hardware

Animations that run smoothly in development but stutter on real devices feel broken and exclude users who cannot afford premium hardware.

### Animating Layout-Triggering Properties

Animating width, height, top, left, or margins forces expensive recalculations and produces jank, especially on mobile.

### Treating Reduced-Motion As A Nice-To-Have

Ignoring or partially implementing the reduced-motion preference leaves users with vestibular conditions exposed to harmful motion.

### Large-Scale Parallax And Zoom

Full-screen parallax, scaling, or panning simulates self-motion and is a leading trigger of motion-induced discomfort.

### Autoplay Loops On Functional Surfaces

Continuous motion behind text, forms, or decisions competes for attention and overwhelms users with sensory or cognitive sensitivity.

### Assuming Motion Will Run Smoothly

Failing to test under load, with real data, and on slow devices lets performance failures reach users undetected.

### No Way To Pause Or Stop Motion

Trapping users in autoplay or looping motion disrespects their attention and fails basic accessibility expectations.

## Self-Check

- [ ] Motion is specified to run on mid-range and low-end devices, animating cheap composited properties such as transform and opacity rather than layout-triggering ones.
- [ ] A reduced-motion alternative is designed and tested for every animation, with minimal spatial travel, and it respects the operating system preference.
- [ ] High-risk vestibular patterns such as full-screen slides, parallax, zoom, and persistent panning are identified, constrained, or removed.
- [ ] Autoplay and looping motion is limited to justified contexts, kept subtle, and pauses when off-screen or on interaction.
- [ ] Continuous or autoplay motion provides a way for the user to pause, stop, or hide it.
- [ ] Motion has been tested on representative slow hardware, with realistic data, under interruption, and in reduced-motion mode.
- [ ] Decorative motion is evaluated against its performance and accessibility cost and removed where it does not earn its place.
- [ ] Functional motion remains fast, cheap, and reduced-motion-aware.
- [ ] No motion traps the user's attention without consent or an escape path.
