---
name: performance_and_critical_css.md
description: Use when the agent is optimizing CSS for load or runtime performance, reducing render-blocking CSS and improving first paint or LCP, extracting and inlining critical above-the-fold CSS while async-loading the rest, removing unused or dead CSS with PurgeCSS or coverage tooling, tree-shaking CSS in a component framework, diagnosing paint and compositing complexity (layers, will-change, overdraw, GPU-layer cost), optimizing animations to use only transform and opacity with FLIP, applying CSS containment with contain and content-visibility, improving font loading (font-display, FOIT/FOUT, subsetting, variable fonts), or measuring CSS performance with Chrome coverage, the rendering profiler, and Core Web Vitals. Use when a page is slow to first paint, janky during animation, or shipping a large stylesheet.
---

# Performance And Critical CSS

CSS performance is unusual because the cost is mostly invisible until it is severe, and the severe cases are specific and diagnosable. A stylesheet that is twice as large as it should be does not crash anything — it adds a few hundred milliseconds to first paint, which quietly depresses conversion and search ranking until someone profiles it. An animation that animates the wrong property does not error — it drops frames and feels "laggy" in a way users notice but rarely report. The judgment problem is that CSS offers many ways to make something *work* and only a few ways to make it *fast*, and the fast ways are not the default. An agent that ships the first working version has usually shipped a performance regression, because the convenient choices (one big render-blocking stylesheet, animating `width`, a `will-change` on every animated element, a full web font with no `font-display`) are each individually reasonable and collectively slow.

This skill is the CSS-specific performance engineering deep dive. It complements `css-architecture-and-specificity`, which covers performance broadly (render-blocking, layout thrash, animating transform), and `web/http-caching-and-browser-fundamentals`, which covers HTTP-level caching. Here the focus is the specialist view: critical-CSS extraction and inlining, dead-CSS removal and the purging dangers, paint and compositing cost, animation performance and the FLIP technique, CSS containment, font loading, and how to *measure* all of it with the right tools rather than optimizing by guess. The recurring discipline is to profile before optimizing — most CSS is not a performance problem, and the cases that matter (first paint, animation smoothness, scroll jank) are identifiable and specific.

## Core Rules

### Understand Why CSS Blocks Rendering, And Treat First Paint As The First Goal

The browser will not render any painted content until the render-blocking CSS in the document has been parsed, because CSS determines layout and paint and rendering without it would produce a flash of unstyled content. The consequence is direct: every byte of render-blocking CSS in `<head>` delays first paint, and therefore delays Largest Contentful Paint (LCP) when the LCP element depends on those styles. A 200 KB render-blocking stylesheet on a slow connection can add seconds to first paint. The two levers are *how much* CSS is render-blocking (ship less, and make the non-critical part non-render-blocking) and *how fast* it arrives (HTTP caching, covered in the http-caching skill; CDN, compression). The first-paint goal is to make the render-blocking CSS contain only what is needed for the initial viewport, and to defer everything else.

### Extract Critical CSS And Inline It; Async-Load The Rest

The critical-CSS strategy is to identify the CSS needed to render above-the-fold content, inline it directly in `<head>` (so it requires no network round-trip and is available for first paint), and load the full stylesheet asynchronously so it does not block rendering. The mechanics:

- **Inline only the critical subset** — the rules actually used by the initial viewport, typically a few kilobytes. Inlining the entire stylesheet defeats the purpose (it bloats the HTML and cannot be cached separately).
- **Load the full CSS without blocking.** Use `<link rel="preload" as="style" onload="this.rel='stylesheet'">`, or load it via a media-query swap (`media="print" onload="this.media='all'"`), so the browser fetches it at low priority and applies it once ready without blocking first paint.
- **Account for the maintenance cost.** Critical-CSS extraction is usually automated (tools that render the page and capture used rules), and the extracted set drifts as the above-the-fold design changes. Treat critical CSS as generated, not hand-maintained, and regenerate it as part of the build; a stale critical set either misses rules (flash of unstyled content) or includes too many (slower first paint).

The tradeoff is real: critical CSS improves first paint but adds build complexity and a flash risk if the extraction is wrong. For a content site where first paint drives revenue, it is usually worth it; for an internal tool where the stylesheet is small and cached, the complexity may not pay off. Decide based on measured first-paint impact, not on the technique's popularity.

### Remove Unused CSS, But Beware The Purging Traps

Shipping CSS that the page never uses is pure cost — it is render-blocking bytes that buy nothing. Removing it (PurgeCSS, UnCSS, coverage-based tooling, or framework-level tree-shaking) is one of the highest-leverage CSS optimizations, especially for sites using large frameworks (CSS frameworks, older UI libraries) where most rules are unused on any given page. But purging has a well-known failure mode: **purgers work by scanning source files for class-name strings, so any class name constructed dynamically (`btn-${variant}`, conditionally applied, or present only in data from an API) is purged as "unused" and silently disappears at runtime.**

- **Safelist dynamic and runtime-generated class names.** Any class not present as a literal string in the scanned source must be safelisted explicitly, or the purger will remove it.
- **Prefer frameworks that tree-shake at the component level** (CSS Modules, CSS-in-JS with build-time extraction, component-scoped styles) where unused styles die with unused components, avoiding the whole-source-scanning problem.
- **Measure with the browser's Coverage panel**, which reports actually-unused CSS against the rendered page, to validate that purging is working and to find what a static purger missed (e.g., rules used only after interaction).

The danger is silent breakage: a purged class produces no error, just a missing style that may not be caught until a user hits the affected state. Test purged output against the full range of component states, not just the default render.

### Animate Only `transform` And `opacity`; Use FLIP For Layout Transitions

The single most important animation-performance rule: animate properties that the compositor handles without recalculating layout or paint. `transform` (translate, scale, rotate) and `opacity` are composited on the GPU and run at 60 fps even on modest hardware. Animating `width`, `height`, `top`, `left`, `margin`, or `padding` triggers layout recalculation on every frame, which is dramatically costlier and produces jank. This is the same principle named in `css-architecture-and-specificity`; the specialist addition is the **FLIP technique** for animating *layout changes* (an element moving from one position/size to another after the DOM changes):

1. **First** — read the element's current position/size (`getBoundingClientRect()`).
2. **Last** — apply the DOM change that moves it to its end state, and read the new position/size.
3. **Invert** — instantly apply a `transform` that visually puts the element back where it was (the inverse of the difference).
4. **Play** — remove the transform via a transition, so the element animates from its old position to its new one using only `transform`.

FLIP lets you animate layout transitions (reordering a list, expanding a card) using only compositing-friendly properties, by doing the expensive layout read once and the animation purely on the compositor. The discipline: any animation that would naturally change layout should be re-expressed as a transform-based animation, with FLIP when the layout genuinely must change.

### Understand Paint, Compositing, And The Cost Of Layers

Beyond layout, the rendering pipeline has two more stages whose cost matters: **paint** (filling pixels) and **composite** (combining layers on the GPU). Properties have different costs:

- **Layout-triggering** (`width`, `height`, `position`, box model) — most expensive; the whole pipeline reruns.
- **Paint-triggering** (`color`, `background`, `box-shadow`, `border-radius`, `outline`) — reruns paint and composite, but not layout. Large blurred shadows and gradients are notably costly to paint.
- **Composite-only** (`transform`, `opacity`) — cheapest; only the compositor runs.

Two related traps: **overdraw** (multiple layers painting the same pixels, e.g., opaque elements stacked over an opaque background) wastes fill work, and **excessive layers** (too many elements promoted to their own GPU layer via `will-change` or implicit triggers) cost memory and compositing time. The `will-change` property is a common offender: it promotes an element to a layer in anticipation of a change, which is correct for a few elements about to animate, but applied indiscriminately (`will-change: transform` on every card) it creates hundreds of layers that exhaust GPU memory and slow compositing. Use `will-change` sparingly, only on elements genuinely about to animate, and remove it when the animation ends. Promote deliberately, not by reflex.

### Use CSS Containment To Isolate Expensive Subtrees

The `contain` property tells the browser that an element's subtree is independent of the rest of the page, so changes inside it do not force layout/paint/invalidation outside it (and vice versa). `contain: layout paint style` (or `content`, a shorthand) on a card, a list item, or a widget lets the browser skip re-laying-out the whole page when that subtree changes. `content-visibility: auto` goes further: it lets the browser skip rendering off-screen subtrees entirely (useful for long lists and feeds), with `contain-intrinsic-size` providing a placeholder size so the scrollbar does not jump.

The discipline: containment is a targeted optimization, not a sprinkle. `contain: content` on a subtree that genuinely isolates is a win; applying it everywhere, or applying `content-visibility: auto` without a correct `contain-intrinsic-size`, produces layout jumps and broken scroll anchoring. Profile to confirm the subtree is actually expensive before containing it, and set intrinsic sizes carefully when using `content-visibility`.

### Optimize Font Loading To Avoid FOIT, FOUT, And Layout Shift

Web fonts are a frequent source of both slow first paint and visible jank. The tools, in order of impact:

- **`font-display`** controls the swap behavior. `swap` shows fallback text immediately and swaps to the web font when ready (FOUT — flash of unstyled text; preferred for performance and perceived speed). `optional` is like swap but may give up the web font entirely on slow connections to avoid a late swap. `block` hides text for up to a few seconds waiting for the font (FOIT — flash of invisible text; bad for LCP and perceived speed). Avoid `block`; prefer `swap` or `optional` based on whether the late swap is acceptable.
- **Subset fonts** to the characters actually used (e.g., a Latin subset, or language-specific subsets) to radically reduce font file size. A full font with thousands of glyphs is often 10x larger than the subset a page needs.
- **Preload the primary font(s)** used by above-the-fold content (`<link rel="preload" as="font">`) so they fetch early and in parallel.
- **Use variable fonts judiciously.** A single variable font can replace several static weights, reducing total bytes, but a variable font file is larger than one static weight — beneficial when you actually need multiple weights/axes, wasteful when you need one.
- **Eliminate layout shift from font swap** by sizing fallback fonts to match the web font's metrics (font-size-adjust, or matching a fallback with similar metrics), so text does not reflow when the web font loads. Unmitigated font swap is a common source of Cumulative Layout Shift (CLS).

### Measure With The Right Tools Before And After Optimizing

CSS performance work without measurement is guesswork, and most guesses are wrong about where the time goes. The tools and what they answer:

- **Chrome DevTools Coverage panel** — reports which CSS rules are unused on the rendered page (and across interactions), validating purging and finding dead CSS.
- **Performance / Rendering profiler** — records a timeline showing layout, paint, and composite costs, and flags forced reflows (layout thrash). Use this to find jank during animation and scroll.
- **Layers panel** — shows the GPU layers the browser created and their cost, useful for diagnosing excessive-layer and overdraw problems.
- **Core Web Vitals (LCP, CLS, INP)** — the user-perceived metrics. LCP is sensitive to render-blocking CSS and font loading; CLS to layout shift including font swap; INP to main-thread work including style recalculation on interaction. Optimize against these, not against synthetic byte counts.

The discipline: capture a baseline metric, make one change, re-measure. Optimizing several things at once makes it impossible to know what helped, and some "optimizations" (extra layers, over-aggressive purging) make things worse.

## Common Traps

### Inlining The Entire Stylesheet As "Critical CSS"

Inlining the whole CSS in `<head>` to avoid a render-blocking request, which bloats the HTML, prevents the CSS from being cached separately, and slows every navigation. Inline only the genuinely critical above-the-fold subset; async-load the rest.

### Stale Critical CSS That Misses Rules Or Includes Too Many

A hand-maintained or rarely-regenerated critical set that either omits rules used above the fold (flash of unstyled content) or includes far more than needed (slower first paint). Treat critical CSS as build-generated and regenerate it as the design changes.

### Purging Dynamically-Constructed Class Names

A purger removing `btn-${variant}` or conditionally-applied classes because they are not literal strings in the scanned source, so styles silently disappear at runtime. Safelist dynamic and runtime-generated classes, or use component-scoped styles that tree-shake with the component.

### Testing Purged Output Only Against The Default Render

Validating purged CSS by loading a page in its default state, missing classes used only after interaction, in a modal, or in an error state. Test against the full range of component states, or use the Coverage panel across interactions.

### Animating `width`/`height`/`top`/`left` Because It Is The Obvious Property

Animating the layout property that visually corresponds to the change, triggering layout on every frame and producing jank. Re-express the animation in `transform`/`opacity`; use FLIP when the layout genuinely must change.

### Sprinkling `will-change` On Every Animated Element and excessive Blurs And Gradients Painted Every Frame

Adding `will-change: transform` to many elements "for performance," which creates excessive GPU layers, exhausts memory, and slows compositing. Use `will-change` only on elements about to animate, and remove it afterward.

Large `box-shadow` blurs and full-screen gradients as backgrounds of animated or frequently-repainted elements, which are costly to paint. Reduce paint area, simplify the effect, or move it to a non-repainting layer.

### Applying `content-visibility: auto` Without `contain-intrinsic-size` and using `font-display: block` To "Avoid" FOUT

Using `content-visibility: auto` on a long list without a placeholder size, so the browser cannot know the off-screen height and the scrollbar jumps as content renders, producing severe CLS. Always set `contain-intrinsic-size` to a realistic estimate.

Setting `block` to prevent a flash of unstyled text, which instead hides text for seconds and harms LCP and perceived speed. Prefer `swap` (or `optional`); eliminate the layout shift from the swap with metric-matched fallbacks instead.

### Shipping A Full Glyph Set When A Subset Would Do and optimizing By Byte Count Instead Of By User-Perceived Metrics

Loading a font with thousands of unused glyphs, multiplying file size. Subset to the characters and languages actually used; preload the primary above-the-fold fonts.

Reducing stylesheet size or layer count without checking LCP/CLS/INP, so the "optimization" does not improve what users actually experience — or, with extra layers and over-purging, makes it worse. Measure the Core Web Vital before and after each change.

## Self-Check

- [ ] Render-blocking CSS was minimized for first paint — only what the initial viewport needs is render-blocking, the rest is async-loaded (`preload`/media-swap) so it does not block rendering — and the decision to use critical-CSS extraction was based on measured first-paint impact, not on the technique's popularity.
- [ ] Critical CSS, where used, is the genuinely above-the-fold subset inlined in `<head>`, is build-generated and regenerated as the design changes (not stale), and the full stylesheet is loaded without blocking rendering.
- [ ] Unused/dead CSS is removed via purging, coverage tooling, or component-level tree-shaking, and dynamic/runtime-constructed class names are safelisted; purged output was tested against the full range of component states, not just the default render.
- [ ] Animations use only `transform` and `opacity` (composited); layout-change animations are re-expressed with transforms, using the FLIP technique (read First, apply Last, Invert, Play) where layout genuinely must change — no `width`/`height`/`top`/`left` animations in hot paths.
- [ ] GPU layers are deliberate: `will-change` is applied only to elements about to animate and removed afterward, not sprinkled everywhere; overdraw (opaque layers over opaque backgrounds) and excessive layers were checked in the Layers panel.
- [ ] CSS containment (`contain`, `content-visibility: auto`) is applied as a targeted optimization to genuinely expensive or off-screen subtrees, with `contain-intrinsic-size` set wherever `content-visibility: auto` is used to prevent scrollbar jumps and CLS.
- [ ] Font loading uses `font-display: swap`/`optional` (not `block`), fonts are subset to the characters/languages used, primary above-the-fold fonts are preloaded, and font-swap layout shift is mitigated with metric-matched fallbacks.
- [ ] Performance changes were measured with the right tools (Coverage panel for unused CSS, Performance/Rendering profiler for layout/paint/composite and forced reflows, Layers panel for layer cost) and validated against Core Web Vitals (LCP, CLS, INP), with a baseline captured before each change so it is clear what helped.
