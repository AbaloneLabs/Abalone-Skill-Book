---
name: animation_state_and_lifecycle_management.md
description: Use when the agent is implementing animation in code — building animation state machines, handling interruptible or reversible animations, managing running animations across component lifecycles, canceling requestAnimationFrame loops, removing event listeners and resize observers, cleaning up on unmount, using animation libraries (Framer Motion, GSAP, Web Animations API, anime.js) and managing their lifecycle, coordinating JavaScript-driven and CSS-driven animations, or reviewing code for leaked animations that cause jank, errors after unmount, or state desync. Also covers the trap of orphaned rAF loops and timers running after a component unmounts, animations that cannot be interrupted or reversed, library animations whose cleanup is forgotten, JS and CSS animations fighting over the same property, and the failure mode of leaked animations causing memory growth, console errors, or visual glitches on navigation.
---

# Animation State And Lifecycle Management

Animation in a real application is long-lived runtime state that must be created, tracked, interrupted, reversed, and torn down across the component lifecycle. Every running animation holds resources — a rAF loop, a timer, a library's internal tracker, event listeners, DOM observers — and each must be released when the animation ends or its owning component unmounts. Agents tend to write animation as if it were self-cleaning, and it often is in the happy path. But lifecycle bugs live in the paths an agent does not test — unmount during animation, rapid retriggers, navigation away mid-transition. Treat every animation as a resource: define its states, make it interruptible, clean up on every exit path, and coordinate it with any other animation touching the same element.

## Core Rules

### Model Each Animation As A State Machine With Defined Transitions

An animation is an object with states — idle, running, paused, finished, cancelled — and the code must handle transitions between them explicitly. Treating animation as implicit ("I called animate, it'll finish") leaves no answer for re-triggers or unmount while paused.

- **Define the states and what triggers each transition.** A panel can be idle, opening, open, closing, closed. Naming them forces you to handle each.
- **Guard against invalid transitions.** If the panel is "opening" and the user triggers "close," the close should begin from the current position, not wait for open to finish.
- **Ensure every state has a defined exit.** A "running" state must reach "cancelled" from anywhere, not only from "finished."

The state machine need not be a formal library — a few variables and clear conditionals suffice. The point is that transitions are deliberate, not accidental.

### Make Animations Interruptible And Reversible From Any Point

The user's intent must always take precedence over in-flight motion. If the user toggles a panel open and immediately toggles it closed, the close must begin from wherever the open animation currently is — not queue behind it, not jump to the open state first.

- **Animate from the current state, not a stored start.** Read the current value (getComputedStyle, the Web Animations API's currentTime, a library's state, or FLIP) so an interruption picks up mid-flight.
- **Cancel the in-flight animation before starting the new one.** Two animations on the same property fight and produce jitter.
- **Support reversal without re-running from the beginning.** A panel half-open toggled closed should animate the remaining half, not restart the open animation first.

The test: trigger an action and immediately trigger its opposite, repeatedly, on a throttled CPU. If it jumps, queues, or lands wrong, it is not interruptible.

### Clean Up Every Animation Resource On Unmount And Cancellation

This is the most important and most missed rule. Every animation resource — a rAF loop, a timer, an event listener, a ResizeObserver/MutationObserver, a library animation handle, a Web Animations API Animation object — must be released when the animation ends, is cancelled, or when its owning component unmounts.

- **Cancel rAF loops in the cancellation path and the cleanup/unmount hook.** A loop that self-stops on "are we done?" is not enough: if the component unmounts mid-loop, the loop keeps firing against a detached node. Store the rAF id and cancel it on cleanup.
- **Clear timers on unmount.** A delayed start or polling interval that fires after unmount runs against a dead component.
- **Remove event listeners and disconnect observers on unmount.** Scroll, resize, pointer, and IntersectionObserver handlers keep firing after the component is gone.
- **Dispose library animation handles.** GSAP tweens/timelines have `.kill()`, Framer Motion has stop/cancel controls, the Web Animations API has `.cancel()`. Call them.

For every resource you create, know where it is released. If you cannot point to the release point, it leaks. Cancellation is not only unmount — it is also "the user triggered a new action," so every early exit must release its resources.

### Coordinate JavaScript-Driven And CSS-Driven Animations On The Same Element

When JS-driven animation (rAF, Web Animations API, a library) and CSS-driven animation (transitions, @keyframes) target the same property, they conflict — each frame one overwrites the other, producing jitter.

- **Do not mix JS and CSS control of the same property simultaneously.** If JS animates `transform`, ensure no CSS transition on `transform` is active.
- **When handing off, make the handoff explicit.** A common FLIP pattern: JS measures and sets the start, then a CSS transition plays, then JS reads the end state. They must not overlap.
- **Pick one owner per property.** A CSS `!important` fighting a JS-set inline style produces unpredictable precedence. The Web Animations API gives a JS object with cancel/compose control — prefer it for programmatic control.

### Choose The Right Animation Primitive And Respect Its Lifecycle Semantics

Different primitives have different lifecycle characteristics. **CSS transitions** are declarative and self-cleaning — cheap for simple changes, but hard to interrupt and give no JS handle to cancel. **CSS @keyframes** run to completion or infinitely; infinite ones must be removed when no longer needed. **The Web Animations API** gives an Animation object with `.cancel()`, `.pause()`, and `currentTime` — the most lifecycle-friendly primitive for programmatic control. **rAF loops** give full control but full responsibility: cancel them on every exit path; use them only when you need per-frame computation. **Libraries (GSAP, Framer Motion, anime.js)** manage much of the lifecycle internally but still need kill calls on unmount. Prefer primitives that self-clean over raw rAF.

### Handle Reduced Motion And Rapid Retriggers In The Lifecycle

Lifecycle robustness is tested by edge cases, not the happy path. Under `prefers-reduced-motion`, the animation may be skipped entirely — but the state must still reach its correct end value. A common bug: reduced motion disables the animation, but because the end state was only set by the animation's completion, the element stays in the start state. Set the final state explicitly. Rapid retriggers must not accumulate animations: ten clicks should not spawn ten rAF loops; each retrigger must cancel the previous first. And navigation away mid-animation must cancel cleanly — a route change that unmounts a component mid-transition must cancel the transition, not leave it running against a detached node.

## Common Traps

### Orphaned rAF Loop Running After Unmount

Starting a `requestAnimationFrame` loop and relying on its own "done" check to stop. When the component unmounts mid-loop, the loop keeps firing against a detached node. The trap is assuming the loop's self-stop is sufficient — unmount can happen at any frame.

### Animating From A Stored Start Value On Interruption

Reading a start value once and animating from it, so an interruption mid-flight jumps back to the stored start. The stored start is stale the moment any interruption occurs — the current rendered state is the only correct origin.

### Two Animations Fighting Over The Same Property

Starting a new animation on a property while a previous one is still running, so each frame they overwrite each other and the element jitters. The trap is assuming two animations on the same property compose — they do not.

### Library Animation Whose Cleanup Is Forgotten

Calling a library's animate function but never calling its kill/cancel/stop on unmount or interruption. The trap is trusting the library to self-clean — it has no knowledge of your component's lifecycle.

### setTimeout/setInterval Or Observer Left Running After Unmount

A delayed start, polling interval, or resize/intersection observer never cleared on unmount. Easy to forget because they are not obviously "animation" — but they fire callbacks against a dead component.

### Reduced-Motion Path That Leaves The Element In The Start State

Disabling the animation under `prefers-reduced-motion` but failing to set the end state explicitly. The trap is assuming the animation's completion will set the end state — when the animation is skipped, that completion never fires, leaving the element stuck at the start value.

### Accumulated Animations From Rapid Retriggers

A control that starts a new animation without cancelling the previous one, so ten rapid clicks spawn ten overlapping animations compounding into jank. Each trigger looks correct in isolation; the bug only appears under rapid repeated use.

## Self-Check

- [ ] Each animation is modeled with defined states (idle, running, paused, finished, cancelled) and guarded transitions, so invalid transitions (retrigger while running, unmount while paused) are handled explicitly.
- [ ] Animations are interruptible and reversible: new input cancels or redirects in-flight motion from the current rendered state (not a stored start), verified on a throttled CPU with rapid retriggering.
- [ ] Every animation resource is released on unmount and cancellation — rAF ids cancelled, timers cleared, listeners removed, observers disconnected, library handles killed — and for each you can point to the release point.
- [ ] JS-driven and CSS-driven animations do not fight over the same property simultaneously; handoffs are explicit, and a property is driven by one source at a time.
- [ ] The animation primitive matches the need (CSS transitions for simple changes, Web Animations API for programmatic control, libraries for orchestration, rAF only with owned cleanup).
- [ ] Under `prefers-reduced-motion`, the animation is skipped but the final state is set explicitly, and rapid retriggers cancel the previous animation rather than accumulating.
- [ ] Navigation away or unmount mid-animation cancels the transition cleanly (no running animation against a detached node, no "set state on unmounted component" errors).
- [ ] The highest-risk cases were verified — orphaned rAF after unmount, interruption from a stored start, two animations fighting over one property, forgotten library cleanup, timers/observers left running, and reduced-motion leaving the start state — not only the happy path.
