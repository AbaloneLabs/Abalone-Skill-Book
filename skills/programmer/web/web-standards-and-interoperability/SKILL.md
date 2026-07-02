---
name: web_standards_and_interoperability.md
description: Use when the agent is writing web code that must work across browsers, deciding a browser support matrix and build target (browserslist), choosing between progressive enhancement and graceful degradation, deciding whether to polyfill or transpile and against which baseline, debugging cross-browser or cross-engine differences (Chromium, WebKit/Safari, Gecko/Firefox, mobile versus desktop), dealing with vendor prefixes or feature gating, evaluating whether an experimental feature behind a flag or origin trial is safe to ship, or judging whether a specification (W3C, WHATWG, TC39) is mature enough to depend on. Also covers the cost of browser-specific workarounds, the Safari lag reality, and the long-term maintenance burden of non-standard or unstable code. Differentiate from browser-apis-and-capabilities which covers using specific APIs, and from http-caching-and-browser-fundamentals which covers HTTP and rendering mechanisms.
---

# Web Standards And Interoperability

The web runs on a constellation of independent engines — Chromium, WebKit (Safari), and Gecko (Firefox), across desktop and mobile — each implementing overlapping but not identical specifications, at different paces, with different bugs. The judgment problem is not "will this work in Chrome" but "what is my support matrix, what baseline works everywhere, where do I enhance, what do I transpile or polyfill and at what cost, and which of these features are stable enough to depend on versus experiments that will rot?" Code that works in the developer's engine and breaks in a major engine is the default outcome when these decisions are made implicitly. Code that depends on an unstable spec becomes a maintenance treadmill. Code that paper-overs every engine quirk becomes unreadable.

Agents tend to target the engine in front of them (usually Chromium), because the demo works and cross-engine testing is effort, and to reach for the newest feature because it is elegant. The harms accumulate: a feature that ships fine in Chrome and silently fails or renders wrong in Safari, which a large share of real users run; a build configured to support ancient browsers that ships megabytes of transpiled output to everyone; a dependency on a spec still at stage 2 that changes and breaks the build on the next browser update; a tangle of `if (isSafari)` branches that no one dares touch. The discipline is to make the support matrix and standards-maturity decisions *deliberately and explicitly*, to build on a stable baseline, and to treat browser-specific workarounds as debt with a removal plan.

This skill is about cross-browser interoperability, standards maturity, and the support-matrix decision. browser-apis-and-capabilities covers the per-API judgment of *using* a capability (workers, observers, permissions); http-caching-and-browser-fundamentals covers the HTTP and rendering mechanisms. Here the focus is: what works where, what is stable enough to depend on, and what it costs to bridge the gaps.

## Core Rules

### Make The Support Matrix And Build Target An Explicit, Owned Decision

Before writing or configuring anything, decide and record which browsers and versions you target. This decision drives everything downstream — transpilation, polyfills, what features you can use directly, and how you test. The mechanism is a **browserslist** configuration (in `package.json` or `.browserslistrc`), which the build toolchain (Babel, SWC, Autoprefixer, ESLint) reads to make consistent decisions.

- **Choose the matrix by your users, not by your laptop.** A consumer-facing site with significant iOS/Safari or older-Android share has a different baseline than an internal tool on evergreen Chromium. Look at real analytics.
- **Express the target declaratively** (`>0.5%, last 2 versions, Firefox ESR, not dead, Safari >= 14`) so the entire toolchain agrees, rather than ad-hoc per-file decisions.
- **Revisit the matrix on a cadence.** Browsers auto-update, share shifts, and old versions die. A matrix set once and never revisited either over-supports (shipping bloat) or under-supports (breaking real users).

The strong practice: the support matrix is written down, drives the build, and is reviewed periodically against real usage. The weak practice is no explicit target, where "it works in my Chrome" becomes the de facto matrix and real users break.

### Build On A Stable Baseline: Progressive Enhancement Over Graceful Degradation

Two philosophies for cross-browser support, and they produce very different outcomes:

- **Progressive enhancement** starts from a baseline that works everywhere (semantic HTML, core functionality without advanced features) and *adds* capability where the browser supports it. The baseline is always usable; capable browsers get more. This tends to be robust because the fallback is the foundation, not an afterthought.
- **Graceful degradation** builds the full advanced experience first, then patches down for weaker browsers. This tends to be fragile because the fallback is bolted on, and an unsupported feature often fails *partially* rather than cleanly.

The strong practice is progressive enhancement for anything with a broad or uncertain matrix: make the core work with the lowest common denominator, then layer enhancements behind feature detection (see browser-apis-and-capabilities). Graceful degradation is acceptable for a narrow, known matrix (an internal tool on a controlled browser set) where the baseline is genuinely the advanced case. The trap is choosing degradation by default and discovering the fallback was never built.

### Tie Transpilation And Polyfilling To The Support Matrix, And Count Their Cost

Transpiling (modern syntax → older syntax) and polyfilling (providing missing platform APIs) are how you bridge the gap between the features you want to use and the browsers you support. Both have real costs, and both must be driven by the same matrix:

- **Transpilation cost.** Targeting older browsers means more syntax transformation, larger output, and more runtime helpers. Targeting only evergreen browsers means you can ship largely untransformed, smaller, faster code. The build target is a performance decision.
- **Polyfill cost.** Every polyfilled API is bytes and parse time. A kitchen-sink polyfill bundle (`core-js` everything) ships megabytes; a usage-based, matrix-scoped polyfill (only what you call, only for browsers that need it) is far smaller. Scope polyfills to actual API usage and to the actual gap.
- **Syntax versus platform.** Transpiling handles *syntax* (optional chaining, etc.); it cannot create *platform APIs* (`IntersectionObserver`, `structuredClone`). Those require polyfills or feature detection plus fallback. Do not assume a transpiler gives you new APIs.

The discipline: set the build target from the matrix, scope polyfills to usage, and periodically raise the baseline as old browsers drop out of the matrix so the bloat shrinks over time. The weak practice is a frozen `> 0%` target that ships maximum polyfills forever.

### Reason About Engine Differences And The Safari Reality

The three engine families are not interchangeable, and ignoring the differences produces "works in Chrome, broken in Safari" bugs that reach a large share of users:

- **Chromium** (Chrome, Edge, most Android browsers, all ChromeOS) is the most widely deployed and usually the most complete, but it is not the spec, and Chromium-first development is the most common source of interop bugs.
- **WebKit** (Safari desktop and iOS, all iOS browsers due to platform rules) has historically lagged and has real gaps and quirks; the "Safari is the new IE" framing is hyperbolic but captures a real risk — iOS WebKit reaches a vast user base and cannot be ignored. Safari also sometimes implements features with prefixes or behind different defaults.
- **Gecko** (Firefox) has smaller share but its own quirks, and on some APIs leads or lags the others.
- **Mobile versus desktop** engines differ even within a family — mobile Safari is not desktop Safari, and mobile memory and CPU constraints change what is feasible.

The strong practice: test in each engine family in your matrix, treat iOS/WebKit as a first-class target (not an afterthought), and never assume Chromium behavior is the spec. The weak practice is "I tested in Chrome, ship it."

### Treat Experimental Features, Flags, And Origin Trials As Not Production-Ready

Browsers expose experimental features behind flags, origin trials, or with vendor prefixes before they are stable. These are tempting and dangerous:

- **Behind-a-flag features** can change, be removed, or never ship. Depending on them in production means your code breaks when the flag flips or the API is redesigned.
- **Origin trials** are time-limited and browser-vendor-gated; they are for feedback, not for shipping to users long-term.
- **Vendor-prefixed features** (`-webkit-`, `-moz-`) are legacy escape hatches from the prefix era; modern practice is to use unprefixed standard properties and let Autoprefixer add prefixes only where still needed. Shipping prefixed CSS/JS as the primary path locks you to engine-specific behavior that may diverge from the eventual standard.

The rule: depend only on features that are shipping unprefixed and unflagged in the engines in your matrix, and treat anything behind a flag, trial, or prefix as experimental with a migration plan to the standard form. The weak practice is reading a flashy blog post about a flag-gated feature and shipping it to users.

### Judge Spec Maturity Before Depending On A Feature

Web specifications mature through stages (TC39 for ECMAScript: stage 0–4; W3C/WHATWG and CSSWG for web platform), and the stage tells you how safe it is to depend on:

- **Stable / shipping (TC39 stage 4, CSS in CR, features in the HTML/WHATWG living standard with multiple-engine implementation)** are safe to depend on with feature detection.
- **Earlier stages and single-engine implementations** are not stable; the spec can change, and a second engine may implement differently. Depending on them creates rework risk.
- **Living standards** (WHATWG HTML) evolve continuously but are generally safe where multiple engines implement; the risk is newer parts not yet widely implemented.

The discipline: before depending on a feature for load-bearing behavior, check its spec stage and its implementation breadth across engines. Prefer features that are stable and multi-engine; treat single-engine or pre-stage features as enhancement only, behind detection, with a fallback. The weak practice is treating "it's in the spec" as sufficient when the spec is early or the implementation is single-engine.

### Treat Browser-Specific Workarounds As Debt With A Removal Plan

Sometimes a workaround for an engine bug or quirk is unavoidable. The discipline is to treat every such workaround as technical debt:

- **Document why it exists** — the specific engine, version, bug, and why no cleaner approach was possible. A bare `if (isSafari)` with no comment is a future mystery.
- **Gate on the symptom, not the browser, where possible** — detect the buggy behavior rather than the UA, so the workaround retires itself when the bug is fixed (see browser-apis-and-capabilities on detection versus UA sniffing).
- **Add a removal trigger** — a linked bug report, a browser-version cutoff, or a date to re-evaluate — so the workaround is deleted when it is no longer needed rather than accumulating forever.

The strong practice: workarounds are rare, documented, symptom-detected, and scheduled for removal. The weak practice is a growing pile of engine branches that no one understands and no one removes, which is how codebases become unmaintainable.

## Common Traps

### Developing And Testing Only In Chromium

Building and verifying only in Chrome/Edge, then shipping, and discovering Safari or Firefox renders differently or lacks a feature a large user share needs. Test in every engine family in your matrix; iOS/WebKit especially is not optional for consumer-facing sites.

### No Explicit Support Matrix, So The Build Target Drifts

No browserslist or target declared, so each tool makes its own default decision, polyfills accumulate ad hoc, and "what we support" is unknowable. Declare the matrix once, drive the whole toolchain from it, and revisit it on a cadence.

### Choosing Graceful Degradation By Default And Never Building The Fallback

Building the full advanced experience assuming everyone supports it, then discovering a feature fails partially in a target browser with no clean fallback. Prefer progressive enhancement so the baseline is the foundation; if you degrade, build and test the degraded path explicitly.

### Shipping A Kitchen-Sink Polyfill Bundle To Everyone

Importing all of `core-js` or a CDN polyfill bundle unconditionally, sending megabytes to browsers that already have the APIs. Scope polyfills to actual usage and to the matrix gap; raise the baseline as old browsers drop out.

### Assuming The Transpiler Provides New Platform APIs

Configuring Babel/SWC and assuming that gives you `structuredClone` or `IntersectionObserver`. Transpilers handle syntax, not platform APIs; those need polyfills or detection-plus-fallback.

### Depending On A Flag-Gated Or Origin-Trial Feature In Production

Shipping code that relies on a feature behind a flag or origin trial, which then changes or is removed, breaking production. Depend only on unprefixed, unflagged, multi-engine features; treat experiments as enhancement-only with a fallback.

### Treating "It's In The Spec" As Sufficient

Depending on a feature because it appears in a spec, when the spec is at an early stage or implemented by only one engine, and hitting rework or interop gaps when it changes. Check spec stage and multi-engine implementation breadth before load-bearing use.

### UA-Sniffing To Branch Around A Quirk

Adding `if (isSafari)` branches that rot as UA strings change and that no one removes after the bug is fixed. Detect the symptom where possible, document the workaround, and add a removal trigger; reserve UA logic for confirmed engine bugs with no detectable symptom.

### Letting Vendor-Prefixed CSS Become The Primary Path

Writing `-webkit-` properties as the main styling and never adding the standard unprefixed form, locking behavior to WebKit and diverging from the eventual standard. Write standard properties and let Autoprefixer add prefixes where still required.

### Accumulating Workarounds Without Removal Triggers

Adding engine-specific branches over time with no documentation or expiration, until the code is a tangle no one will touch. Every workaround gets a reason, a symptom-based gate where possible, and a date or version to re-evaluate.

## Self-Check

- [ ] An explicit support matrix is declared (e.g., a browserslist configuration) and drives the entire build toolchain (transpilation, polyfills, autoprefixing, linting), it was chosen from real user analytics rather than the developer's laptop, and it is scheduled for periodic review as browser share shifts.
- [ ] Cross-browser support follows progressive enhancement — a stable baseline works across the whole matrix and enhancements are layered behind feature detection — rather than graceful degradation with fallbacks bolted on after the fact or never built.
- [ ] Transpilation and polyfilling are scoped to the matrix and to actual API usage (not a kitchen-sink bundle), the distinction between syntax (transpiled) and platform APIs (polyfilled or detected) is respected, and the baseline is raised as old browsers drop out so bloat shrinks over time.
- [ ] The code was tested in every engine family in the matrix — including iOS/WebKit, not just Chromium — and known engine differences (Safari lag, Gecko quirks, mobile versus desktop) were checked rather than assumed away.
- [ ] No production code depends on flag-gated, origin-trial, or vendor-prefixed features as a primary path; experiments are enhancement-only behind detection with a fallback, and prefixed CSS has the standard unprefixed form as the base.
- [ ] Each load-bearing feature was checked for spec maturity (TC39 stage, CSSWG/W3C/WHATWG status) and multi-engine implementation breadth before dependence, and single-engine or pre-stability features are treated as enhancement-only rather than relied upon.
- [ ] Every browser-specific workaround is documented (engine, version, bug, reason), gated on the detectable symptom where possible rather than UA sniffing, and has a removal trigger (linked bug, version cutoff, or re-evaluation date) so it is deleted when no longer needed rather than accumulating as unmaintainable debt.
