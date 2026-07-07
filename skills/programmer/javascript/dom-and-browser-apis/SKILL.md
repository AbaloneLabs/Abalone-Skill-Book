---
name: javascript_dom_and_browser_apis.md
description: Use when the agent is writing browser JavaScript that touches the DOM, document lifecycle, events, forms, storage, history, intersection/resize observers, Web Components, custom elements, shadow DOM, requestAnimationFrame and layout thrashing, or is diagnosing reflow/repaint cost, event delegation bugs, memory leaks from detached nodes and listeners, form submission and validation issues, focus management, or "the element is null" timing bugs. Covers DOM manipulation judgment, event and lifecycle correctness, and the performance and memory pitfalls of browser APIs.
---

# DOM And Browser APIs In JavaScript

The DOM is a live, shared, mutable tree that the browser continuously reconciles against a rendering pipeline. Treating it like a plain data structure — querying nodes, mutating them, attaching listeners — works for toy examples and produces subtle, expensive, leak-prone code in real applications. Three classes of failure dominate DOM work. First, timing: the element you query may not exist yet because the script ran before the parser reached it, or it may be replaced by a later render, leaving a stale reference. Second, cost: every layout-affecting read interleaved with a write forces the browser to flush its render queue (layout thrashing), and large synchronous DOM work blocks the main thread and drops frames. Third, lifecycle and memory: event listeners, observers, closures, and detached nodes retain references that the garbage collector cannot reclaim, producing leaks that grow over a long session. The judgment problem is to treat the DOM as a managed surface with a lifecycle, not as a global mutable bag of nodes.

Agents frequently reach for direct DOM manipulation (`document.getElementById`, `innerHTML =`, `addEventListener`) because it is the path of least resistance, and then ship code that works in a fresh tab and degrades over a session: listeners pile up, detached subtrees leak, frames drop during scroll, and a re-render silently invalidates a cached node reference. The remedy is not to abandon the DOM but to make every interaction deliberate — query at the right time, batch reads and writes, scope listeners to a lifetime, and clean up observers and listeners when their owner is removed.

## Core Rules

### Query The DOM Only After It Is Ready, And Re-Query When It Can Change

A script in `<head>` or before the target element runs before the node exists; `document.getElementById` returns `null`, and the classic "cannot read properties of null" error follows. Place scripts at the end of `<body>`, use `defer`, or run initialization inside `DOMContentLoaded` (or wait for the specific element). More subtly, in any framework or re-rendering setup, a node reference captured once can become stale when the subtree is re-rendered; re-query at the point of use, or use event delegation so the handler does not hold a specific node.

- Prefer `defer` on scripts, or `type="module"` (which is deferred by default), so the DOM is parsed before the script runs.
- If you cache a node reference across re-renders, treat it as possibly stale and re-query on use, or scope it to a subtree you control.
- For dynamic lists, prefer event delegation on a stable parent over per-item listeners, so adding/removing items does not require re-binding listeners.

### Avoid Layout Thrashing By Batching Reads And Writes

The browser batches DOM writes and only computes layout when a read forces it. Interleaving a layout read (`.offsetWidth`, `.getBoundingClientRect`, `.scrollTop`, `getComputedStyle`) with a write (`style.x = ...`, `appendChild`) forces a synchronous layout (reflow) on every iteration, which is O(n) per access and can turn a loop into seconds of jank.

- Read all layout-affecting values first, then perform all writes, instead of alternating inside a loop.
- For animations and continuous updates, use `requestAnimationFrame` so writes are synchronized with the frame, and never `setInterval`/`setTimeout` for visual work.
- Prefer CSS transforms and opacity for animation (GPU-composited, no layout) over `top`/`left`/`width` which trigger layout and paint.

### Prefer Event Delegation Over Per-Element Listeners

Attaching a listener to every item in a list costs memory and requires re-binding when the list changes. A single delegated listener on a stable ancestor handles events for current and future descendants via event bubbling, and it naturally tracks dynamic content. Delegate when items are added/removed dynamically or when the list is large; attach directly when the element is stable and few.

- In a delegated handler, use `event.target.closest(selector)` to find the relevant element, and guard against matches outside the list.
- Remember that some events (`focus`, `blur`, `mouseenter`, `mouseleave`, `submit` in some cases) do not bubble; use the capturing phase or the focusin/focusout bubbling equivalents for delegation.

### Clean Up Listeners, Observers, Timers, And Detached Nodes

The browser cannot reclaim memory held by: a listener still attached to a node in the document, a listener attached to a detached node that something still references, an active `IntersectionObserver`/`ResizeObserver`/`MutationObserver`, a pending `setInterval`/`setTimeout`, or a closure capturing a large node. Each is a leak that grows across a session.

- Remove listeners when their owner is removed (`removeEventListener` requires the same function reference and options; an inline anonymous function cannot be removed — store the reference).
- Call `observer.disconnect()` for Intersection/Resize/Mutation observers when the component is destroyed.
- Clear intervals and timeouts on teardown.
- `AbortController` is the modern way to remove a batch of listeners and abort fetches at once: pass its `signal` to `addEventListener` and call `controller.abort()` on teardown.
- Avoid retaining references to removed nodes in long-lived data structures; a map keyed by node keeps the node and its subtree alive.

### Prefer textContent Over innerHTML, And Build With The DOM API

`innerHTML` re-parses HTML, is a cross-site scripting (XSS) vector when the content includes user data, and destroys existing listeners and state inside the replaced subtree. Use `textContent` for plain text, `document.createElement` + `append` for structured building, and a trusted sanitization step (or a framework with automatic escaping) when you must insert HTML.

- Never concatenate user input into `innerHTML`. This is the single most common XSS source in hand-written DOM code.
- `Element.insertAdjacentHTML` is more targeted than wholesale `innerHTML` replacement but has the same XSS concern; sanitize first.

### Handle Forms As Submission-Centered, Not Just Input-Centered

Forms submit. A button without `type="button"` inside a form submits the form; Enter in a text field submits the form; a non-AJAX submission reloads the page. Design form handling around the `submit` event (with validation and `preventDefault` for AJAX submission), use semantic inputs and labels for accessibility, and rely on native validation (`required`, `pattern`, `type`) before adding custom logic.

- Bind to `form.addEventListener('submit', ...)` and `preventDefault()` for client-side handling, not to individual button clicks.
- Associate `<label>` with inputs (via `for`/`id` or wrapping) for accessibility and larger hit area.
- Disable the submit button during in-flight submission to prevent double submission.

### Manage Focus, Scroll, And Viewport Deliberately

Focus management (initial focus, focus trap in dialogs, restoring focus on close) and scroll restoration affect usability and accessibility. `IntersectionObserver` is the efficient way to lazy-load or react to visibility; `ResizeObserver` is the efficient way to react to size changes. Both replace polling and `scroll`/`resize` listeners that fire far too often.

- Use `IntersectionObserver` for lazy loading and reveal-on-scroll instead of `scroll` + `getBoundingClientRect`.
- Throttle or debounce `scroll`/`resize` listeners if you must use them, and prefer passive listeners (`{ passive: true }`) so they do not block scrolling.

## Common Traps

### Querying A Node Before It Exists

A script runs, calls `getElementById`, gets `null`, and crashes on the next line. Run after `DOMContentLoaded` or use `defer`/module scripts; do not assume the parser has reached the element.

### Layout Thrashing In A Loop

```
for (const el of items) { const w = el.offsetWidth; el.style.width = w + 10 + 'px'; }
```
forces a layout every iteration. Read all widths first, then write all styles.

### innerHTML With User Content (XSS)

`el.innerHTML = '<p>' + userInput + '</p>'` executes scripts and injects markup when `userInput` contains HTML. Use `textContent`, or sanitize with a trusted library.

### Leaked Listeners On Removed Nodes

A listener attached to a node that is later removed from the DOM still keeps that node (and its subtree) alive if anything references the listener or node. Remove listeners on teardown, or use delegation so no per-node listener exists.

### Stale Node Reference After Re-Render

A reference captured once (`const btn = document.querySelector(...)` in a framework that re-renders) points to a node no longer in the document after the re-render. Re-query, or use delegation.

### Non-Bubbling Events In Delegation

`focus`/`blur`/`mouseenter`/`mouseleave`/`submit`(sometimes) do not bubble, so a delegated listener on a parent never fires. Use `focusin`/`focusout` (which bubble) or the capturing phase.

### setInterval For Animation

`setInterval(fn, 16)` does not sync with the display refresh, runs in background tabs, and piles up callbacks if the work is slow. Use `requestAnimationFrame` for visual work.

### Forgetting passive Listeners Block Scrolling

A non-passive `scroll` or `touchmove` listener forces the browser to wait for the handler before scrolling, producing jank. Add `{ passive: true }` unless you genuinely need to `preventDefault()`.

## Self-Check

- [ ] Scripts run after the DOM is ready (`defer`, module, or `DOMContentLoaded`), and node references that can be invalidated by re-render are re-queried at use or replaced by delegation.
- [ ] Layout reads and writes are batched rather than interleaved in loops, animations use `requestAnimationFrame` and transform/opacity, and no `setInterval` drives visual updates.
- [ ] Dynamic lists use event delegation on a stable parent, and non-bubbling events are handled via bubbling equivalents or the capture phase.
- [ ] Listeners, observers (`disconnect()`), timers, and `AbortController` signals are cleaned up on teardown, and no detached node is retained by a long-lived structure.
- [ ] No user content is concatenated into `innerHTML`; `textContent` or sanitized insertion is used, and the XSS surface has been audited.
- [ ] Forms are handled on the `submit` event with `preventDefault` for AJAX, semantic inputs and labels are used, and double submission is prevented.
- [ ] Focus, scroll, and visibility use `IntersectionObserver`/`ResizeObserver`/passive listeners rather than polling, and focus is managed for dialogs and dynamic views.
- [ ] The DOM interaction has been considered under re-render, long session, slow device, and teardown, and remains correct and leak-free.
