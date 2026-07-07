---
name: performance_and_bundle_optimization.md
description: Use when the agent is reducing bundle size, code splitting routes or components, configuring tree shaking, lazy loading heavy dependencies, optimizing images and fonts, auditing third-party scripts, measuring or improving Core Web Vitals (LCP, INP, CLS), analyzing webpack/rollup/vite bundle output, deciding what to defer or preload, or diagnosing a slow app on low-end devices and mobile networks. Also covers critical rendering path, unused JavaScript, duplicate dependencies, dynamic imports, prefetch versus preload, render-blocking resources, the cost of shipping a large JavaScript bundle, and the failure mode of an app that is fast on the developer's machine but unusable on a budget phone. This skill is about what ships to the browser and how to measure and trim it, and is distinct from rendering-strategies (which decides when and where HTML is produced) and from component-level re-render concerns.
---

# Performance And Bundle Optimization

Frontend performance is a budget, not a feature. Every kilobyte of JavaScript, every render-blocking stylesheet, every third-party tag, and every unoptimized image is paid for in parse time, main-thread work, battery, and data cost — and the bill is paid on the user's device, not the developer's. The judgment problem is that "fast" looks fine in development: the engineer runs the app on a powerful laptop over a fast connection, the bundle is cached, the images are local, and nothing feels wrong. The harm only appears later, on a mid-range Android phone on a 3G connection, where a 2MB JavaScript bundle takes ten seconds to parse before the page is interactive, where a 4MB hero image pushes LCP past four seconds, and where three analytics tags block the main thread so long that every tap has visible lag.

Agents tend to treat performance as a polish step added at the end, or as a single metric to hit, rather than as a set of tradeoffs reasoned about continuously. Adding a date library "because it's convenient" ships 70KB to show a calendar; importing an entire icon set to use three glyphs ships megabytes; a charting library loaded eagerly on the landing page blocks interaction for users who never scroll to the chart. The discipline is to measure first, then cut what is not needed, defer what is not immediately needed, and ship the smallest viable payload — while tracking the metrics that reflect real user experience, not the metrics that are easiest to improve.

This skill is about what is shipped to the browser and how to measure and trim it. It is distinct from rendering-strategies, which decides when and where HTML is produced (SSR/SSG/CSR/streaming) and the hydration that follows; here the render mode is assumed chosen, and the question is what bytes go down the wire and how fast they become a usable page.

## Core Rules

### Measure With Real Metrics And Track The Core Web Vitals That Map To User Experience

Performance work without measurement is guesswork, and most guesses are wrong. Establish a baseline using two complementary lenses: lab data (Lighthouse, WebPageTest, throttled runs) for a reproducible score to regress against in CI, and field data / RUM (real-user Core Web Vitals) for the truth. A page that scores 95 in Lighthouse can still fail the 75th-percentile mobile user. Field data is the truth; lab data is the guardrail. Never declare a win based on a local run, and never optimize a metric you are not measuring in the field.

Each Core Web Vital maps to a dimension users actually feel, and optimizing the wrong one wastes effort. **LCP (Largest Contentful Paint)** is perceived load, dominated by the hero image or large text block — improved by server response, image optimization, font loading, and removing render-blocking resources. **INP (Interaction to Next Paint)** is responsiveness, driven by main-thread blocking from long JavaScript tasks and heavy hydration — a high-INP page feels sluggish on every tap. **CLS (Cumulative Layout Shift)** is visual stability, caused by images and embeds without reserved dimensions, late-loading fonts, and dynamically injected content. The decisive question: which vital is failing, and is the work targeting its cause? Optimizing bundle size helps INP and LCP; reserving image dimensions fixes CLS; deferring scripts does nothing for an LCP bound by the hero image. Match the fix to the failing metric.

### Treat The JavaScript Bundle As The Primary Cost Center

JavaScript is the most expensive byte on the web: it must be downloaded, parsed, compiled, and executed on the main thread, and that cost is paid on every load, disproportionately on low-end devices. A page can be "small" in raw kilobytes and still be slow because the JavaScript blocks the main thread. Manage the bundle deliberately.

- **Know what is in it.** Run a bundle analyzer and read it. Large blocks are usually a heavy library, a full icon set imported wholesale, a moment/locale bundle, or duplicate copies of a dependency from version mismatches.
- **Eliminate unused and duplicate dependencies.** Tree shaking removes dead code only when modules are ES modules imported in a way that allows it (named imports, no whole-module side effects). A barrel file with side effects, or a CommonJS dependency, silently defeats tree shaking and ships the whole library.
- **Prefer smaller alternatives only when justified.** Measure the real delta and weigh the maintenance cost of a bespoke implementation; do not rewrite a well-tested library to save 5KB.

The strong default is to ship less JavaScript. Every library added should justify its bytes against the value it provides on the pages where it loads.

### Split Code So Each Route Ships Only What It Needs

A single monolithic bundle forces every user to download the code for every route, including ones they will never visit. Code splitting breaks the bundle into chunks loaded on demand, so the initial load contains only what the landing route needs. **Route-level splitting** should be the baseline, not an optimization added later: each route is a dynamic import. **Component-level splitting** applies to heavy, late, or conditional UI — a charting library, a rich text editor, a modal, a below-the-fold widget — lazy-loaded when first needed. The test: is this code needed for first render or first interaction? If not, split it. Chunk shared vendor code sensibly so splitting does not duplicate shared dependencies across many tiny chunks. The trade is initial-load size for a small latency cost on navigation, and it is almost always worth it: the user who never visits a route should not pay for it.

### Defer, Lazy-Load, And Protect The Critical Rendering Path

The critical rendering path is the minimum set of resources needed to render above-the-fold content; everything else should be deferred so it does not block that first paint. Apply the right tool by what the resource is. **Defer non-critical JavaScript** (analytics, tag managers, chat widgets, below-the-fold scripts) with `defer` or `async`, or inject it after the page is interactive — render-blocking scripts in `<head>` are a common, avoidable LCP and INP cost. **Lazy-load images** outside the viewport with `loading="lazy"` and reserve their dimensions to prevent layout shift, while the hero image — usually the LCP element — is prioritized (`fetchpriority="high"`). **Load fonts without blocking text** using `font-display: swap` (or `optional` for non-critical fonts) and preload the critical font files, so text renders immediately and swaps without reflow. **Preload only what is truly critical and certain** — the LCP image and critical fonts — and prefetch the next likely route only when the prediction is reliable, since over-using hints wastes bandwidth and can starve the critical fetch. The principle: the browser should spend its early bandwidth on exactly what is needed to paint the first useful content, and nothing else.

### Audit And Contain Third-Party Scripts

Third-party scripts — analytics, tag managers, A/B testing, chat, ads, social embeds — are frequently the largest single performance cost on a page, and they are often added by non-engineers with no review of their impact. They download their own JavaScript, run on the main thread, and can block rendering and interaction long after your own code is done. **Measure each tag's cost** with a tool that attributes main-thread time and blocking to each script; a single tag manager can add seconds of INP-blocking work that is invisible until you look. **Defer or lazy-load** third-party scripts wherever possible — most analytics and marketing tags do not need to load before the page is interactive, and should never load synchronously in the head. **Prefer server-side or first-party alternatives** where they exist, which move the script off the user's device entirely and are often the single biggest win on high-traffic pages. **Gate tags by environment and consent** so scripts do not load in development and consent-gated tags do not load until consent is given. Third-party scripts are the part of the payload the team least controls and most often ignores; treat adding a new tag as a performance decision requiring measurement, not a configuration change.

## Common Traps

### Optimizing The Wrong Metric Or Judging Only On The Developer's Machine

Improving the Lighthouse score by trimming CSS when the real problem is INP from a heavy main-thread script, or declaring the app fast based on a local run over a fast connection. Look at the field data for the failing vital, target its actual cause, and validate on throttled network and constrained CPU profiles representative of real devices.

### Shipping A Library Or Icon Set For A Narrow Use

Adding a full date, utility, or UI library to use one function, or `import { Icon } from 'huge-icon-library'` that pulls in the whole set because of how the package is structured. Check the bundle impact before adding a dependency; use per-file or named imports from packages that support tree shaking, and watch for CommonJS or side-effect-bearing barrels that defeat it.

### Eagerly Loading Heavy Below-The-Fold UI

Bundling a charting library or rich editor into the initial chunk when it only renders in a section the user has not scrolled to. Lazy-load heavy, conditional, or below-the-fold components so the initial load does not pay for them.

### Render-Blocking Third-Party Scripts In The Head

A tag manager or analytics script loaded synchronously in `<head>`, blocking first paint and first interaction, while the team assumes the slowness is "the framework." Defer or async-load non-critical third-party scripts, measure each tag, and prefer server-side alternatives.

### Unoptimized Hero Image And Layout Shift

Serving a multi-megabyte hero image without priority hints so the LCP element loads last, alongside images without reserved dimensions and late fonts that reflow text — each contributing CLS. Optimize and responsively size the LCP image, reserve dimensions for all media, and use stable font-loading strategies.

## Self-Check

- [ ] A performance baseline was established using both lab data (Lighthouse/WebPageTest, gated in CI) and field/RUM data from real users, and no performance claim rests on a local run alone.
- [ ] The failing Core Web Vitals were identified from field data, and the optimization work targets the actual cause of each failing metric (LCP from hero image/fonts/blocking resources, INP from main-thread JavaScript, CLS from unreserved dimensions and font reflow).
- [ ] The JavaScript bundle was analyzed and is deliberately small: unused and duplicate dependencies were removed, tree shaking is effective (no whole-module or side-effect-bearing imports defeating it), and every heavy library justifies its bytes.
- [ ] Code is split at the route level by default, and heavy, conditional, or below-the-fold components (charts, editors, modals, widgets) are lazy-loaded rather than bundled into the initial payload, with shared vendor code chunked sensibly to avoid duplication.
- [ ] The critical rendering path is protected: non-critical JavaScript is deferred or async-loaded, below-the-fold images are lazy-loaded with reserved dimensions, the hero/LCP image is prioritized, and fonts load without blocking or reflowing text.
- [ ] Every third-party script was measured for its main-thread and blocking cost, deferred or lazy-loaded where possible (or moved server-side/first-party), gated by environment and consent, and adding a new tag is treated as a measured performance decision.
- [ ] Performance was validated on constrained profiles (throttled network, slowed CPU) representative of low-end devices and mobile networks, not only on the developer's fast machine.
