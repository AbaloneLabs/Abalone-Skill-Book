---
name: browser_apis_and_capabilities.md
description: Use when the agent is using advanced browser platform APIs and capabilities, offloading main-thread work to Web Workers or Service Workers, building offline or PWA behavior, deciding whether to use WebAssembly for compute-heavy or ported code, requesting device permissions (camera, microphone, geolocation, notifications) and handling denial, implementing lazy loading or infinite scroll with Intersection Observer, responsive component sizing with Resize Observer, choosing between feature detection and user-agent sniffing, deciding whether to polyfill or progressively enhance, or reasoning about whether a browser feature is safe to ship given capability versus support versus real-world bugs. Also covers performance budgeting against API capabilities and the risk of assuming a present API works correctly. Differentiate from http-caching-and-browser-fundamentals which covers the HTTP and rendering mechanisms, and from web-standards-and-interoperability which covers cross-browser support decisions.
---

# Browser APIs And Capabilities

The modern browser is not a document viewer; it is a capable platform with workers, compiled modules, device sensors, observers, and a permission system. The judgment problem is not "does this API exist" but "is leveraging it the right call, given its lifecycle cost, its integration boundary, the permission friction it adds, and the gap between *shipped* and *actually reliable*." A Web Worker that nobody tears down, a Service Worker whose update flow was never designed, a WebAssembly module that doubles first-load time for a marginal compute win, a permission prompt that fires on page load and scares users away — these are the costs of treating platform APIs as free.

Agents tend to reach for a capable API the moment they learn it exists, because the demo works and the capability feels powerful. The invisible costs appear later and are severe: a Service Worker that caches aggressively and then serves stale content forever because the update lifecycle was never wired; a feature that "is supported" on caniuse but is buggy or half-implemented in a major engine; a permission request that silently degrades to denial and leaves a feature broken with no fallback; an Observer that fires on a hot path and creates the very jank it was meant to remove. The discipline is to treat each platform capability as a contract with lifecycle, permission, performance, and reliability obligations — and to fall back gracefully when the contract cannot be honored.

This skill is about *leveraging* browser platform APIs and capabilities judiciously. The HTTP- and rendering-level mechanisms (caching headers, CORS, web vitals) are covered by http-caching-and-browser-fundamentals; the cross-browser support-matrix decision (browserslist, vendor prefixes, spec maturity) is covered by web-standards-and-interoperability. Here the focus is the per-API judgment: workers, WASM, permissions, observers, detection, and polyfills.

## Core Rules

### Treat Each Capability As A Lifecycle And Integration Contract, Not A Function Call

Before adopting any non-trivial platform API, identify its lifecycle obligations and the integration boundary. Most capability bugs are lifecycle bugs.

- **Web Workers** offload work to a separate thread, but they are not free. A worker has startup cost, a serialized message boundary (`postMessage` copies or transfers structured data), and must be explicitly terminated. The strong practice is to use a worker when you have sustained CPU work (parsing, hashing, image processing) that would otherwise block the main thread for tens of milliseconds or more, and to keep the message boundary coarse (send a batch, not a stream of tiny messages). The weak practice is to spawn a worker for a one-off trivial computation where the message overhead exceeds the work.
- **Service Workers** are a programmable network proxy that runs independently of page lifetime. Their power is also their danger: a service worker intercepts fetches and can serve cached responses *forever* unless you design the update flow. You must handle the install/activate lifecycle, version the cache, and implement a strategy for activating new service-worker versions (e.g., `skipWaiting` plus a reload prompt, or controlled activation). A service worker with no update story is a future outage waiting to happen.
- **SharedWorker and Worklets** have narrower, more fragile support and stricter isolation rules; reach for them only when their specific capability is required and you have verified the support matrix.

The rule: name the lifecycle (creation, message boundary, update, termination) before writing the code. If you cannot describe when the capability is torn down and how it updates, you are not ready to ship it.

### Decide WebAssembly On Total Cost, Not Microbenchmarks

WebAssembly (WASM) lets you run compiled code in the browser and is genuinely transformative for compute-heavy work (video/audio processing, games, simulation, porting existing native libraries). The judgment is about *total cost*, because WASM has real overheads that a single benchmark hides:

- **Load and compile cost.** A WASM module must be downloaded, decoded, and compiled, and large modules (multi-megabyte) can dominate first-load. Streaming compilation and caching help, but the bytes still travel.
- **The integration boundary.** Crossing between JS and WASM has cost; data must be marshalled into linear memory. A tight loop that bounces between JS and WASM per call can be *slower* than equivalent JS.
- **Memory management.** WASM manages its own linear memory; you must size it, grow it, and avoid leaks. Porting a C library that assumes abundant heap can blow the browser's memory budget.

The strong practice: use WASM when there is a sustained, hot, well-bounded computation (or an existing native library worth porting), measure the end-to-end impact including load, and keep the JS↔WASM boundary coarse. The weak practice is to rewrite ordinary application logic in Rust→WASM "for performance" without measuring, shipping a large module to shave microseconds off work that was never the bottleneck.

### Design Permission Flows Around User Denial From The Start

Camera, microphone, geolocation, notifications, and similar capabilities require explicit user permission, and the user may deny it — permanently, silently, or in a way the page cannot distinguish. The discipline is to design the *denial path* first, not the happy path:

- **Check the Permissions API** (`navigator.permissions.query`) before prompting, so you know whether permission is granted, denied, or prompt, and avoid re-prompting a user who already said no.
- **Prompt only in response to a clear user gesture and a clear value explanation.** A permission prompt that fires on page load is perceived as surveillance and is frequently denied forever. Tie the request to a button the user clicked after understanding what they get.
- **Handle every permission outcome explicitly** — granted, denied, prompt, and the "permission revoked later" case. A feature that assumes permission will be granted is a feature that breaks for a meaningful fraction of users with no fallback.
- **Never assume permission persists.** Users can revoke permissions from browser settings. Re-check at the point of use, and degrade gracefully.

The strong practice: the feature works (or fails clearly) in all four permission states. The weak practice is a geolocation feature that throws an uncaught exception when the user denies, leaving a blank map and no explanation.

### Prefer Observers Over Scroll And Resize Event Listeners

For visibility and sizing, the platform provides purpose-built Observers that are dramatically more efficient and correct than the legacy event listeners:

- **Intersection Observer** reports when elements enter or leave the viewport (or a configurable root), coalesced and off the hot scroll path. Use it for lazy loading images, infinite scroll triggers, and visibility analytics. The legacy alternative — a `scroll` listener calling `getBoundingClientRect` on every element — janks the page and fires far too often.
- **Resize Observer** reports when an element's content box changes size, which is what responsive components actually need (the *element* resized, not just the window). The legacy `window.resize` event only fires on viewport change and misses element-level resizes caused by content, font loading, or container changes.

The rule: reach for the Observer first; use the legacy event only when you have a specific reason the Observer cannot serve (and verify that reason). The performance and correctness gap is large enough that the Observer is almost always correct.

### Detect The Feature, Never The User-Agent

User-agent strings are unreliable, actively being frozen by browsers, and historically the source of endless compatibility bugs (sites sniffing "Chrome" and breaking real browsers, or sniffing version numbers that lie). The correct approach is feature detection:

- **Detect the specific capability you intend to use**, e.g. `'IntersectionObserver' in window` or `CSS.supports('aspect-ratio', '1 / 1')`. This tests the thing you actually depend on.
- **Use `@supports` in CSS** to gate style on a real capability rather than a browser name.
- **Avoid inferring behavior from UA** ("if Safari, then..."). Engines converge and diverge in ways UA strings do not capture; the only reliable signal is whether the feature works.

The rare legitimate use of UA is for *workaround gating* when a feature exists but is confirmed buggy in a specific engine and no feature-detectable signal distinguishes the bug — and even then, prefer detecting the bug's symptom when possible. Default to detection; treat UA sniffing as a last resort with a comment explaining why.

### Decide Polyfills Versus Progressive Enhancement Versus Transpile Deliberately

When a target browser lacks a capability, you have three distinct strategies, and they are not interchangeable:

- **Progressive enhancement** builds a baseline that works everywhere and adds capability where present. Best when the enhancement is optional (a nicer animation, an offline mode) and the baseline is genuinely usable. Costs nothing for old browsers; the new browser simply gets more.
- **Polyfilling** provides the missing API so the same code runs everywhere. Best when the API is stable and the polyfill is faithful and small. Costs download and parse time for every browser that includes it, and a poor polyfill can diverge from the spec.
- **Transpiling** (Babel, SWC) converts newer syntax to older equivalents. Best for syntax, not for new platform APIs (transpiling cannot create a real `IntersectionObserver`).

The judgment: polyfill only what your *actual support matrix* requires (tie this to browserslist, see web-standards-and-interoperability), prefer the platform's native behavior where present, and avoid bundling kitchen-sink polyfills that ship megabytes for features you never use. Be cautious of third-party polyfill CDNs (the polyfill.io incident showed the supply-chain risk); vendor and pin what you depend on.

### Treat Capability As Distinct From Support, And From Reliability

A feature being "supported" on caniuse means it exists in the engine; it does not mean it is correct, complete, or performant. Three separate questions:

- **Capability** — does the API exist at all in this browser? (Feature detection answers this.)
- **Support breadth** — across how much of your support matrix? (See web-standards-and-interoperability.)
- **Reliability** — does it actually work correctly, including edge cases, in every engine that "supports" it? (Only real testing answers this.)

A feature may ship but be buggy in WebKit, partial in Gecko, or behave differently under specific conditions. Before depending on a capability for anything load-bearing, verify it against the engines in your matrix with real tests, and check known issues. The gap between "supported" and "reliable" is where production bugs live.

### Budget Capability Against Performance And User Value

Every capability has a cost: bytes to download, parse, and execute; memory to hold; permissions to request; complexity to maintain. The discipline is to budget:

- **Justify the cost against user value.** A Service Worker that enables real offline use may be worth its complexity; a Service Worker added "because PWA" with no offline design is pure cost.
- **Measure the capability's contribution to web vitals.** A large WASM module inflates LCP and TBT; an over-eager Observer can create INP regressions. Tie capability adoption to measured impact (see http-caching-and-browser-fundamentals on web vitals).
- **Defer capability loading** until needed. A camera library should not be in the initial bundle if the camera feature is behind a button.

## Common Traps

### Shipping A Service Worker With No Update Strategy

Registering a service worker that caches the app shell, then never designing how a new version activates. Users get stuck on stale code, deploys appear not to ship, and "clear cache" becomes the only fix. Always implement and test the update lifecycle before relying on a service worker in production.

### Reaching For WASM To Optimize Code That Was Not The Bottleneck

Rewriting business logic in WASM "for speed" without profiling, then shipping a large module that slows first load while the actual bottleneck was a render-blocking stylesheet or a layout thrash. Measure end-to-end before adopting WASM; the bottleneck is rarely where intuition points.

### Prompting For Permission On Page Load

Requesting camera, location, or notifications immediately, before the user understands the value. Conversion collapses and denials become permanent. Tie permission requests to a user gesture with a clear value explanation, and check the Permissions API first.

### Assuming A Denied Permission Will Be Re-Promptable

Treating denial as temporary. Once a user denies (or dismisses enough times), the browser may not show the prompt again, and the page gets a silent denied state. Always handle denial with a clear fallback and, where appropriate, instructions to re-enable in settings.

### Using A Scroll Listener Where Intersection Observer Belongs

Wiring infinite scroll or lazy loading to a `scroll` event with `getBoundingClientRect`, creating main-thread work on every frame and missing coalescing. The result is jank and battery drain that the Observer eliminates for free.

### Feature-Sniffing The Wrong Thing

Detecting `'serviceWorker' in navigator` and assuming the *entire* service-worker surface (background sync, periodic sync, push) is available. Sub-features vary. Detect the specific capability you depend on, not a broad umbrella that may be partially present.

### UA-Sniffing Because "It's Easier"

Branching on `navigator.userAgent` for a workaround, then breaking when a browser updates its UA string or a new engine appears. Feature detection survives browser evolution; UA sniffing rots. Reserve UA logic for confirmed, documented engine bugs with no detectable symptom.

### Bundling A Kitchen-Sink Polyfill

Importing `core-js` (or a CDN polyfill bundle) unconditionally, shipping megabytes of polyfills for features you never use, to browsers that already have them. Scope polyfills to your real support matrix and to the APIs you actually call.

### Trusting "Supported" Without Testing Reliability

Reading caniuse green and shipping, then discovering the feature is buggy or partial in a major engine. Capability ≠ support ≠ reliability. Test against the real engines in your matrix before depending on a capability for load-bearing behavior.

### Leaking Worker Resources

Spawning Web Workers per interaction without terminating them, accumulating background threads and memory. Workers have lifetime; terminate or reuse them deliberately, and never assume the page closing is the only teardown path (service workers and shared workers outlive the page).

## Self-Check

- [ ] For each non-trivial platform API adopted (Web Worker, Service Worker, WASM, Observer), the lifecycle is explicitly designed — creation, message/integration boundary, update flow, and termination/teardown are all described, and no worker or service worker is left without a teardown or update story.
- [ ] WebAssembly was chosen on the basis of end-to-end measurement (load and compile cost, JS↔WASM boundary marshalling, memory sizing) against a real bottleneck, not a microbenchmark, and the module is loaded lazily if it is not needed for first paint.
- [ ] Every permission-gated capability (camera, mic, geolocation, notifications) handles all four states — granted, denied, prompt, and revoked — prompts only after a user gesture with a clear value explanation, and checks the Permissions API before prompting so already-denied users are not re-asked.
- [ ] Visibility-dependent behavior (lazy loading, infinite scroll, analytics) uses Intersection Observer, and element-resize-dependent behavior uses Resize Observer, rather than `scroll` or window `resize` listeners on hot paths.
- [ ] Capability gating uses feature detection (`'X' in window`, `CSS.supports`, `@supports`) rather than user-agent sniffing, and any remaining UA-based branch is documented with the specific engine bug it works around and why no feature-detectable signal exists.
- [ ] Polyfills are scoped to the actual support matrix and the specific APIs used (tied to browserslist), not a kitchen-sink bundle, and any third-party polyfill CDN is vendored and pinned rather than loaded live.
- [ ] Each adopted capability was verified for *reliability* (real tests in each engine in the support matrix), not just presence, and known bugs or partial implementations in major engines were checked before depending on it for load-bearing behavior.
- [ ] Capability adoption was budgeted against performance and user value — large modules are deferred, Observers and workers do not regress INP or inflate LCP/TBT, and the capability's cost is justified by the user-facing value it enables.
