---
name: frontend_performance_and_core_web_vitals.md
description: Use when the agent is optimizing web frontend performance, improving Core Web Vitals (LCP, CLS, INP), diagnosing slow page loads or janky interaction, deciding on code splitting or lazy loading, sizing bundles, evaluating third-party scripts, choosing between synthetic and RUM monitoring, or reviewing rendering strategy (CSR/SSR/SSG/streaming). Covers the Largest Contentful Paint, Cumulative Layout Shift, and Interaction to Next Paint metrics and their drivers, the critical rendering path, resource prioritization and preload/prefetch, deferred and lazy loading, bundle and asset optimization, the hidden cost of third-party scripts and iframes, the distinction between lab (synthetic) and field (RUM) data, and the discipline of optimizing the metrics real users experience on their devices and networks. Also use when a page is slow on mobile or slow networks, when Lighthouse scores are good but field data is bad, or when deciding whether a rendering framework migration is justified for performance.
---

# Frontend Performance And Core Web Vitals

Frontend performance is governed by a small set of user-experienced metrics, and the recurring failure is optimizing something other than those metrics. A team ships a fast-feeling dev environment, a high Lighthouse score, and a clever framework — and real users on mid-range Android over a flaky 3G connection still wait six seconds for the largest element to paint and fight layout shifts while it does. The gap exists because the metrics that are easy to measure locally (load time on a fast laptop over fiber) are not the metrics that govern the user experience in the field, and the optimizations that improve local feel (a big SPA bundle, client-side rendering, third-party analytics) are often the ones degrading field performance. Core Web Vitals exist precisely to force attention onto what real users experience: when the main content becomes visible, how much the layout jumps around, and how quickly the page responds to input.

Agents tend to optimize the visible machinery — bundle size, framework choice, a clever loading animation — rather than the specific things that move LCP, CLS, and INP, because the machinery is what the code controls directly and the metrics are downstream and harder to reason about. The defects live in the resource waterfall: the LCP element blocked behind a render-blocking stylesheet or a late-discovered image; the CLS caused by images without dimensions, late-injected ads, or web fonts swapping in; the INP caused by long tasks on the main thread, third-party scripts, or event handlers that do too much synchronously. The judgment problem is treating the Core Web Vitals as the target and reasoning backward to the specific resources and main-thread work that drive them, on the devices and networks real users have, measured in the field rather than only in the lab.

This skill is about improving the performance users actually experience on the web. It complements the prioritization skill (deciding what to optimize) and the benchmarking skill (measuring correctly); here the question is what makes web frontends fast or slow for real users, and how to move the metrics that matter.

## Core Rules

### Target The Core Web Vitals As The Primary User-Experienced Metrics

LCP, CLS, and INP are the metrics that correlate with how fast and smooth a page feels, and they are what search and user perception reward. Optimize them directly, not proxies.

- **Largest Contentful Paint (LCP)** measures when the largest visible element (often a hero image, heading, or video poster) finishes rendering. Good is under 2.5s on the 75th percentile of field loads. LCP is driven by server response time (TTFB), render-blocking resources, the LCP element's load time, and client rendering cost. Identify the LCP element, then remove everything between the user's request and that element painting.
- **Cumulative Layout Shift (CLS)** measures unexpected layout movement over the page's life. Good is under 0.1. CLS is driven by images and embeds without reserved space, dynamically injected content (ads, banners) pushing content, and web fonts causing FOIT/FOUT shifts. Reserve space for every async or sized element before it loads.
- **Interaction to Next Paint (INP)** measures the latency of user interactions (clicks, keypresses), tracking the worst (effectively) across the page's life. Good is under 200ms. INP is driven by long tasks on the main thread blocking input handlers, third-party scripts, and heavy synchronous handler work. Keep the main thread free; break up long tasks; defer non-critical work.

These three are the targets. A page that loads fast in the lab but scores poorly on field LCP/CLS/INP is slow where it matters.

### Reason Backward From The Metric To Its Specific Drivers

Each vital has a small set of drivers, and optimizing a driver that does not affect the metric is wasted effort. Identify which driver is the bottleneck before changing anything.

- **For LCP, find the LCP element and trace the path to it.** Is the server slow (TTFB)? Is a render-blocking CSS or sync script delaying first paint? Is the LCP image discovered late (loaded by CSS or client-side JS rather than in the initial HTML)? Is the image itself large or unoptimized? Fix the specific link that delays the LCP element.
- **For CLS, find what shifts.** Use field and lab tooling to identify shifting elements. Almost always it is images/embeds without dimensions, late-injected content, or fonts. Each has a known fix (reserve dimensions, reserve space for injections, use font-display and matching metrics).
- **For INP, find the long tasks.** Profile the main thread during interaction. Long tasks are usually third-party scripts, heavy event handlers, or framework re-renders. Move work off the main thread (web workers), break long tasks with yielding (scheduler, setTimeout), and defer what is not needed for the interaction.

### Optimize The Critical Rendering Path And Resource Prioritization

The browser can only paint once it has the HTML, the render-blocking CSS, and the content. Every resource that blocks first paint, and every late-discovered resource needed for the LCP element, delays the metrics that matter.

- **Eliminate render-blocking resources on the critical path.** Inline critical CSS or preload it; defer or async non-critical scripts; avoid synchronous scripts in the head that delay parsing and first paint.
- **Prioritize the LCP resource.** If the LCP element is an image, ensure it is discoverable in the initial HTML (not loaded via CSS or client JS) and high-priority; use `fetchpriority="high"` and preload it. A late-discovered LCP image is a top cause of poor LCP.
- **Use preload for critical, late-discovered resources** and prefetch for likely-next resources. Reserve preload for resources the browser cannot discover early (fonts, the LCP image if loaded indirectly); over-using preload de-prioritizes everything and helps nothing.
- **Defer and lazy-load below-the-fold and non-critical resources.** Images below the fold, third-party widgets, and non-critical scripts should load after the main content. But lazy-load with care: do not lazy-load the LCP element, and ensure lazy images still have dimensions to avoid CLS.

### Control Bundle Size And Asset Delivery

JavaScript is the most expensive frontend resource: it must download, parse, compile, and execute, all on the main thread, on devices far slower than the developer's laptop. Bundle size directly affects time-to-interactive and INP.

- **Ship less JavaScript.** The fastest code is the code not shipped. Question every dependency; prefer smaller alternatives; remove unused code (tree-shaking, dead code elimination). A heavy framework or component library may cost more than it is worth.
- **Code-split by route and by interaction.** Load the code for the current route, and defer code for features behind an interaction until needed. Avoid shipping one large bundle for the entire app.
- **Optimize images and media.** Images are usually the largest bytes and often the LCP element. Serve modern formats (WebP, AVIF), right-size and responsive-srcset them, and compress. Video should be lazy and adaptive.
- **Beware the cost of hydration and client-side rendering.** SSR/SSG with heavy hydration can ship HTML fast but then block the main thread rehydrating, harming INP. Streaming and partial/island hydration reduce this; choose a rendering strategy that serves the metrics, not the framework's default.

### Account For The Hidden Cost Of Third-Party Scripts

Third-party scripts (analytics, tag managers, ads, A/B testing, chat widgets, social embeds) are frequently the dominant cost in real-world page weight and main-thread time, and they are often added without performance review.

- **Audit third parties regularly.** Each tag, pixel, and embed adds requests, bytes, and main-thread work. Inventory them, justify each, and remove what is not essential.
- **Defer or lazy-load third-party scripts.** Load them after the main content and the user's first interaction; use async/defer and partytown-style off-main-thread execution where possible.
- **Reserve space for third-party embeds.** Ads, embeds, and widgets injected late cause CLS. Reserve their dimensions before they load.
- **Recognize that one tag manager can load dozens of scripts.** A single tag manager entry can pull in a large fraction of the page's cost; review what it actually loads, not just the manager itself.

### Measure In The Field (RUM), Not Only In The Lab (Synthetic)

Lab data (Lighthouse, local testing) is controlled and reproducible but runs on a fast machine over a simulated network, so it systematically under-represents the experience of real users on slower devices and networks. Field data (Real User Monitoring, RUM) captures what actual users experience, including the long tail.

- **Use lab data to diagnose, field data to judge.** Lab tooling is good for finding specific issues (a render-blocking script, an unoptimized image) under controlled conditions. Field data tells you whether the metrics are actually good for your users.
- **Field data reveals the long tail.** The 75th percentile of field LCP/CLS/INP is what matters, not the median; the median can be fine while a quarter of users have a poor experience. Optimize the tail, which is usually mobile and slow-network users.
- **A good Lighthouse score with bad field data means the lab does not represent your users.** Reconcile the two by testing on representative devices (mid-range Android) and networks (throttled 3G/4G), and trusting field data as the source of truth.
- **Segment field data.** Aggregate RUM can hide that a specific route, device class, or geography is far worse. Segment by page, device, connection, and geography to find where the bad experiences concentrate.

## Common Traps

### Optimizing A Proxy Metric Instead Of The Vital

Pursuing bundle size or a Lighthouse score in isolation, without confirming LCP/CLS/INP moved for real users. Target the vitals directly; use proxies only as they correlate with the vitals.

### Lazy-Loading The LCP Element

Applying lazy-loading indiscriminately, including to the hero image or heading that is the LCP element, delaying the most important paint. Never lazy-load above-the-fold critical content; preload and prioritize the LCP resource.

### Images And Embeds Without Reserved Dimensions

Loading images, ads, or embeds without width/height (or aspect-ratio), so when they arrive the layout jumps, inflating CLS. Reserve space for every async or sized element before it loads.

### Good Lab Score, Bad Field Experience

A high Lighthouse score on a fast laptop over fiber, while real users on mid-range mobile over 3G have poor LCP. Trust field (RUM) data as the source of truth; test on representative devices and networks.

### Ignoring The Long Tail

Reporting the median LCP/INP while the 75th percentile (the threshold that matters) is poor, hiding that a quarter of users have a bad experience. Track and optimize the tail, segmented by device and network.

### Third-Party Scripts Added Without Performance Review

Adding analytics, tag managers, ads, and widgets one by one until they dominate page weight and main-thread time, never audited. Inventory and justify each third party; defer or off-thread them.

### Heavy Hydration After Fast HTML

Shipping SSR/SSG HTML that paints fast, then blocking the main thread rehydrating a large app, harming INP. Choose rendering and hydration strategies (streaming, islands, partial) that serve the metrics, not the framework default.

### Optimizing On A Fast Laptop Over Fiber

Developing and testing only on a fast machine and network, so the experience of the majority of users (slower devices, slower networks) is invisible. Test on mid-range devices and throttled networks regularly.

## Self-Check

- [ ] The primary targets are the Core Web Vitals (LCP under 2.5s p75, CLS under 0.1, INP under 200ms) measured in field (RUM) data, not lab scores or proxy metrics; the long tail (p75, segmented by device/network/geography) is tracked, not just the median.
- [ ] For each vital, the specific driver was identified before changing anything: the LCP element and the resource path to it, the shifting elements causing CLS, and the long tasks blocking INP — not a generic "make it faster."
- [ ] The critical rendering path is clear of render-blocking resources, the LCP resource is discoverable in initial HTML and prioritized (preload/fetchpriority), and below-the-fold/non-critical resources are deferred or lazy-loaded (but never the LCP element).
- [ ] Bundle size is controlled (less JavaScript shipped, code-split by route and interaction, dependencies justified and tree-shaken), images use modern formats with responsive srcset and compression, and the chosen rendering/hydration strategy serves the metrics rather than the framework default.
- [ ] Third-party scripts are inventoried and justified, deferred or off-main-thread where possible, and their embeds have reserved dimensions to prevent CLS; a tag manager's actual loaded scripts are reviewed, not just the manager entry.
- [ ] Lab (synthetic) data is used to diagnose specific issues and field (RUM) data is the source of truth for whether users actually have a good experience; discrepancies (good lab, bad field) are reconciled by testing on representative devices (mid-range mobile) and networks (throttled).
- [ ] Field data is segmented by page, device, connection, and geography to find where poor experiences concentrate, and the optimization targets the segments and percentiles that are actually poor.
- [ ] Improvements were confirmed in field data (the metric real users experience), not only in lab tooling, before being declared successful.
