---
name: render_and_layout_performance.md
description: Use when the agent is designing layouts and interactions whose implementation cost affects rendering performance, including long lists, scroll behavior, animations, sticky and fixed elements, shadows and blur, layout thrash, virtualization, containment, and choices that cause reflow, repaint, or jank on real devices.
---

# Render And Layout Performance

Some designs are cheap to render and some are expensive, and the difference is rarely visible in a static frame. A layout can look identical in a prototype yet perform completely differently in the browser or app, depending on whether it triggers constant reflow, repaints large areas, animates properties that force layout, or stacks expensive effects like blur and shadow. Designers who do not understand render cost produce interfaces that feel sluggish, janky, or unresponsive on the devices where most users live.

Use this skill when designing scrolling surfaces, animations, dense lists, sticky elements, and effects-heavy interfaces. The goal is to prevent the agent from choosing patterns that look smooth in a tool but degrade into frame drops and input lag in production, and to make render cost a design consideration alongside visual quality.

## Core Rules

### Understand The Difference Between Reflow, Repaint, And Composite

Rendering work is not equal. Some changes force the browser to recompute layout, which is the most expensive operation. Others only repaint pixels, which is cheaper. The cheapest changes only composite already-painted layers. Designers do not need to write the code, but they must understand which category their choices fall into.

Rough cost ordering, most to least expensive:

- **Layout or reflow**: changing size, position, or content of elements forces the browser to recompute the geometry of affected and surrounding elements. Examples: changing width, height, font size, inserting or removing elements.
- **Paint or repaint**: changing appearance without changing geometry, such as color, background, or shadows. Cheaper than reflow but still costly over large areas.
- **Composite**: transforming or opacity changes on a promoted layer. Cheapest, can run on the GPU.

Animations and interactions should prefer composite-friendly properties, transform and opacity, and avoid animating layout-triggering properties like width, height, top, or left.

### Animate Transform And Opacity, Not Layout

The most common performance mistake in motion design is animating properties that trigger layout. Animating width, height, margin, padding, top, or left forces reflow on every frame and drops frames on all but the fastest devices.

Prefer:

- `transform: translate` for movement instead of changing position;
- `transform: scale` for resizing instead of changing width or height;
- `opacity` for fade instead of changing visibility or color;
- `transform: rotate` for rotation.

When a design calls for an element to grow or slide, specify it as a transform so the implementation can stay on the compositor. If layout animation is unavoidable, limit it to small, isolated areas.

### Be Cautious With Expensive Effects

Visual effects like blur, large shadows, and filters are expensive, especially when applied over large areas or animated. A single blurred backdrop can dominate frame time on a low-end device.

Evaluate effect cost:

- backdrop blur over large areas is among the most expensive common effects;
- large or multiple box shadows repaint broadly when the element changes;
- filters applied to imagery reprocess pixels continuously when animated;
- mix-blend-mode over large regions forces recomposite of stacked layers.

Use expensive effects sparingly, prefer them on small surfaces, and avoid animating them. Where a frosted-glass look is desired, consider whether a static fallback serves low-end devices.

### Design Long Lists And Scrolling Surfaces For Virtualization

Long lists are a classic performance failure. Rendering thousands of rows at once consumes memory and slows the main thread, causing sluggish scroll and input lag. The solution, virtualization, only renders the visible items plus a buffer, but it imposes design constraints.

When designing scrollable lists:

- keep item heights predictable where possible, since variable heights complicate virtualization and scrollbar accuracy;
- avoid sticky or parallax elements inside the scroll container that fight virtualization;
- design for the fact that off-screen items are not in the DOM, so tooltips, menus, and focus must account for items appearing and disappearing;
- provide stable identifiers so selection and state survive recycling;
- avoid heavy per-item effects or nested scroll containers that multiply cost.

A beautiful list design that cannot be virtualized will jank at scale.

### Limit Sticky, Fixed, And Overlapping Layers

Sticky headers, fixed sidebars, floating action buttons, and overlapping layers each create compositing layers and repaint work. A few are fine; stacking many, especially with effects, multiplies cost and can cause jank during scroll.

Check:

- how many sticky or fixed elements exist on a single screen;
- whether overlapping translucent layers force constant recomposite;
- whether sticky elements contain effects like blur that repaint on every scroll frame;
- whether fixed elements cover content and force the layout to compensate.

Prefer fewer, simpler persistent layers. Each one is a continuous cost, not a one-time one.

### Avoid Layout Thrash From Read-Then-Write Patterns

Some interactions force the browser into a pattern where it must measure layout, then change it, then measure again, repeatedly within a frame. This is layout thrash, and it is often caused by design patterns that require constant measurement, such as resize observers that restyle, or animations that read positions and write styles.

Design choices that invite thrash:

- elements that must measure siblings to position themselves;
- scroll-driven animations that read scroll position and write layout properties;
- dynamic sizing that depends on measuring content then setting explicit dimensions;
- JavaScript-driven positioning that runs on every scroll event.

Prefer CSS-driven solutions and compositor-friendly properties that do not require measurement loops.

### Provide Reduced-Motion And Static Alternatives

Animations that improve perceived performance on capable devices can harm it on weak ones, and some users disable motion entirely. Design motion as an enhancement with a static fallback.

Ensure:

- meaningful state changes are understandable without animation;
- reduced-motion preferences are respected;
- the static state does not depend on the animation completing to be usable;
- motion is not the only way feedback is communicated.

### Test Against Low-End Reference Devices

Render performance cannot be judged on a fast laptop. A smooth 60fps animation on a developer machine can run at 15fps on an entry-level phone. Establish a low-end reference device and review animations, scroll, and effects there.

Reference device testing should cover:

- scroll smoothness on long lists;
- animation frame rate during transitions;
- input responsiveness during background work;
- effect performance, especially blur and shadow.

## Common Traps

### Animating Layout Properties

Animating width, height, top, or left forces reflow every frame and drops frames on real devices, even when it looks fine in a prototyping tool.

### Backdrop Blur Everywhere

Frosted glass is attractive but is among the most expensive common effects, especially when large or animated. Using it pervasively harms low-end devices.

### Rendering Entire Long Lists

Designing lists without considering virtualization produces memory bloat and scroll jank at scale.

### Stacking Many Sticky Layers

Multiple sticky and fixed elements with effects multiply compositing cost and cause scroll jank.

### Judging Performance On A Fast Machine

A design that feels smooth on a developer laptop can be unusable on the devices most users actually have.

### Assuming CSS Animations Are Always Cheap

CSS animations are cheap only when they animate compositor-friendly properties. Animating layout-triggering properties in CSS is just as expensive as in JavaScript.

### Ignoring Scroll-Driven Repaints

Sticky elements with shadows or blur repaint on every scroll frame, turning a continuous cost into constant jank.

## Self-Check

- [ ] Animations use transform and opacity, not layout-triggering properties like width, height, or top.
- [ ] Expensive effects like backdrop blur and large shadows are used sparingly, on small areas, and not animated.
- [ ] Long lists and scrollable surfaces are designed to support virtualization, with stable identifiers and predictable item structure.
- [ ] The number of sticky, fixed, and overlapping layers is minimized, and none carry expensive effects.
- [ ] Interaction patterns avoid layout thrash from constant measure-then-write loops.
- [ ] Reduced-motion and static alternatives exist so the interface is usable without animation.
- [ ] Animations, scroll, and effects were reviewed on a low-end reference device, not only a fast laptop.
- [ ] Scroll-driven repaints from sticky or effect-bearing elements were considered and mitigated.
