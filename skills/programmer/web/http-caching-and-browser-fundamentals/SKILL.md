---
name: http_caching_and_browser_fundamentals.md
description: Use when the agent is setting HTTP caching headers (Cache-Control, ETag, Last-Modified, Vary, Age), configuring CDN or browser caching for assets or API responses, deciding cache-busting and content fingerprinting for static assets, handling conditional requests and revalidation (304 Not Modified), caching authenticated or personalized responses safely, dealing with CORS and same-origin policy, reasoning about the browser rendering pipeline, or diagnosing stale-asset, mixed-content, or CORS failures. Also covers cache hierarchies (browser, proxy, CDN), the distinction between freshness lifetime and validation, public vs private caching, immutable assets, service workers and the Cache Storage API, cookie behavior, web vitals (LCP/CLS/INP), and the security boundary of cross-origin resource sharing. Use when a deploy serves stale assets, when a CDN caches the wrong user's data, when a fetch fails with a CORS error, when optimizing web vitals, or when designing how a resource should be cached across the HTTP and browser stack. Application-level cache strategy and invalidation are covered by the caching-strategy skill; this skill focuses on the HTTP- and browser-level mechanisms.
---

# HTTP Caching And Browser Fundamentals

Every resource a browser loads — an HTML document, a script, an image, an API response — travels through a stack of caches (browser, proxy, CDN) whose behavior is governed by HTTP headers the server sets once and that every hop interprets. The judgment problem is not "should this be cached" but a chain of decisions: *at which layer, for how long, revalidated how, keyed by what, and visible to whom — and what happens when the cached copy and the origin disagree?* Getting this chain right is the difference between a site that loads instantly and one that re-downloads everything on every visit, or between a correct deploy and one where users see last week's JavaScript for days.

Agents tend to set caching headers casually or copy a snippet, because a page "works" without thinking about caching and the cost is invisible until it bites. The bites are specific and severe: a deploy where half the users see the new HTML but the old JS bundle, producing broken pages and inexplicable errors; a CDN that caches an authenticated response under a public key and serves one user's data to another; an asset named `app.js` cached for a year that cannot be updated because the browser will never ask again; a CORS error that appears only in production because the preflight was never tested. The discipline is to treat every resource's caching and cross-origin behavior as a deliberate, reviewed contract — because the browser and the HTTP stack will faithfully execute whatever contract you wrote, including the wrong one.

This skill covers the HTTP- and browser-level mechanisms. The *strategy* of what to cache and how to keep it correct at the application layer is covered by the caching-strategy skill; here the focus is the headers, the cache hierarchy, the browser's security and rendering model, and the failure modes that live at this layer.

## Core Rules

### Set Deliberate Cache Headers For Every Resource — None Is A Decision Too

The absence of caching headers is not "no caching"; it is "the browser and proxies guess," and they guess inconsistently. Every resource should have an explicit caching contract. The primary lever is `Cache-Control`, whose directives compose:

- **`max-age=<seconds>`** — the freshness lifetime: how long the cache may serve the copy without revalidating. This is the single most important value to set deliberately.
- **`public` vs `private`** — `public` allows shared caches (CDN, proxy) to store the response; `private` restricts storage to the end user's browser. **`private` is the correct default for anything personalized**, because a shared cache storing it risks cross-user leakage.
- **`no-cache`** — despite the name, the cache *may* store the response, but it must revalidate with the origin before each reuse. Good for content that must be fresh but avoids refetching when unchanged.
- **`no-store`** — do not store at all. The strictest directive; appropriate for truly sensitive or always-fresh data, but costly if overused (it forfeits all caching benefit).
- **`immutable`** — promises the resource will never change during its freshness lifetime, so the browser need not revalidate even on reload. Pair with content-hashed filenames (see below).

The strong practice: every response carries an explicit `Cache-Control`, and the choice reflects the resource's nature — immutable hashed assets cached aggressively, HTML documents revalidated, personalized responses `private` or `no-store`. A response with no cache headers is an unmade decision that the stack will make for you, badly.

### Use Content Fingerprinting For Immutable Assets, Not Filename Stability

The reliable way to cache static assets (JS, CSS, images, fonts) for a long time *and* be able to update them is content fingerprinting: include a hash of the file's content in its filename (`app.a3f9c1.js`), set `Cache-Control: max-age=31536000, immutable`, and never change that file's contents. When the content changes, the hash changes, the filename changes, and it is a brand-new resource the browser fetches fresh.

- **Hashed filenames let you cache forever safely.** Because the name encodes the content, a cached copy is always correct for its lifetime, and `immutable` is truthful.
- **Do not cache un-hashed filenames aggressively.** `app.js` cached for a year means the browser never asks for it again, so updates never reach users. If you cannot hash, use short `max-age` with revalidation so updates propagate.
- **The HTML document must not be cached long.** HTML is the entry point that references the hashed assets; if it is cached for a year, users never see new asset references. Cache HTML with `no-cache` (revalidate) or a short `max-age`, so a deploy's new HTML is seen promptly and pulls the new hashed assets.

The common broken-deploy pattern — new code shipped, users see errors — is almost always an HTML/asset caching mismatch: the HTML was cached (stale references) or the assets were un-hashed and cached (stale code). Fingerprinted assets plus revalidated HTML eliminates it.

### Understand The Two Caching Modes: Freshness (Expiration) And Validation

HTTP caching operates in two modes, and confusing them is the root of most caching bugs:

- **Freshness / expiration (`max-age`, `Expires`).** The cache serves the copy directly without contacting the origin until the freshness lifetime ends. Fast (no network at all, or a 304), but the copy may be stale for up to the lifetime. This is "cache it and don't ask."
- **Validation / revalidation (`ETag`, `Last-Modified`, triggered by `no-cache` or expiry).** The cache holds the copy but asks the origin "is this still valid?" before reusing it. The origin replies `304 Not Modified` (copy is good, no body re-sent) or `200` (here is the new version). This is "ask before using."

The combination is powerful: a long `max-age` for speed with an `ETag` so that when it expires, revalidation is cheap. The trap is misreading which mode is active: setting `no-cache` and expecting freshness (it revalidates every time, slower than expected), or setting a long `max-age` and expecting freshness (it serves stale until expiry, updates delayed). Know which mode each resource is in and whether staleness or freshness-cost is the acceptable tradeoff.

### Treat Personalized And Authenticated Responses As A Security Surface At This Layer

This is the HTTP-level instance of a general cache-safety rule: a shared cache (CDN, proxy) that stores a response keyed only on the URL will serve User A's authenticated page to User B. The defenses at the HTTP layer:

- **Mark personalized responses `private`** so shared caches will not store them; only the user's browser may.
- **`Vary` by every request header that changes the response** — `Authorization`, `Cookie`, `Accept-Language`, `Accept-Encoding`. A response that differs by any of these must list it in `Vary`, or a cached variant is served to a request that expects a different one. Omitting `Vary: Authorization` on an authenticated response is a cross-user leak.
- **Default authenticated responses to `no-store` unless you have explicitly designed per-user caching.** When in doubt, do not cache authenticated content at shared layers.

Cross-user cache leakage at the HTTP/CDN layer is a recurring high-severity bug. Treat any cacheable authenticated response as needing an explicit, reviewed decision about `private`, `Vary`, and whether shared caching is permitted at all.

### Reason About The Cache Hierarchy: Browser, Proxy, CDN

A request may pass through several caches, each with its own store, and a response's effective caching is the composition of all of them. Reason about where a copy lives:

- **Browser cache** — private to the user; controlled by `private` and the user's settings; the only cache for `private` responses.
- **Shared / proxy caches** — corporate proxies, ISP caches; store `public` responses and serve them to different users.
- **CDN / edge cache** — the most powerful shared cache for latency and origin protection, but also the most dangerous for personalized content if mis-scoped.

A `public, max-age=3600` response may be served from any of these for an hour without touching the origin. That is the goal for static/public content and a hazard for personalized content. When debugging "users see stale data" or "users see another user's data," identify which layer holds the copy — browser, proxy, or CDN — because the fix differs (the user can hard-reload the browser; a CDN purge is a separate operation).

### Handle CORS And Same-Origin Policy As A Security Boundary, Not A Nuisance

The same-origin policy is the browser's fundamental security boundary: by default, a script from origin A cannot read responses from origin B, and cross-origin requests are restricted. CORS (Cross-Origin Resource Sharing) is the controlled relaxation of that policy. It is widely misunderstood because it is enforced by the *browser*, not the server, and because it has two modes:

- **Simple requests** (GET/HEAD/simple POST with simple headers) are sent directly; the browser allows the request but blocks the *response* from being read unless the server returns the right `Access-Control-Allow-Origin`.
- **Preflighted requests** (with custom headers, non-simple methods, or non-simple content types) trigger a preflight `OPTIONS` request first; the browser sends the real request only if the preflight approves. A missing or wrong preflight response is the classic "CORS error" in production.

The discipline: when a frontend on one origin calls an API on another, configure the API to send the correct CORS headers (`Access-Control-Allow-Origin`, `Allow-Methods`, `Allow-Headers`, and `Allow-Credentials` when cookies are involved), test the preflight explicitly, and understand that `Allow-Credentials: true` combined with `Allow-Origin: *` is forbidden — you must echo the specific origin. Never treat CORS errors as browser bugs; they are the security model working, and the fix is almost always a server-side header.

### Respect Cookie And Authentication Mechanics In Caching And Requests

Cookies travel with requests to their origin automatically, and this interacts with caching and CORS in ways that cause subtle bugs:

- **Cookies affect cacheability.** A response that varies by cookie/authenticated state must be `private` and `Vary` on the relevant header, or the CDN serves one authenticated variant to all.
- **`SameSite` and `Secure`/`HttpOnly` govern cookie behavior** — `SameSite=Lax/None` affects whether cookies are sent on cross-site requests, which interacts with CORS and CSRF defenses. A `SameSite=None` cookie requires `Secure`.
- **Credentials in CORS require explicit opt-in** on both sides (`Allow-Credentials: true` on the server, `credentials: 'include'` on the fetch). A fetch that "loses" the cookie usually omitted the credential opt-in.

### Ground Performance In The Rendering Pipeline And Web Vitals

Browser-level performance is governed by the rendering pipeline (parse HTML → build DOM, parse CSS → build CSSOM, execute JS, layout, paint, composite) and measured by the web vitals that map to user-perceived performance:

- **`<script>` placement and `defer`/`async`** determine whether JS blocks parsing. Render-blocking scripts in `<head>` delay first paint; `defer` lets parsing continue.
- **LCP (Largest Contentful Paint)** measures perceived load — usually dominated by a large image, hero text, or a render-blocking chain. Optimize the loading of the largest above-the-fold element.
- **CLS (Cumulative Layout Shift)** measures visual stability — caused by images without dimensions, late-loading fonts, or injected content. Reserve space for async content.
- **INP (Interaction to Next Paint)** measures responsiveness — dominated by long JavaScript tasks blocking the main thread.

The rule: measure the vitals on real devices and networks, find the dominant contributor, and address the pipeline stage it maps to. Optimizing without measurement (e.g., "make JS smaller") often misses the actual bottleneck (a render-blocking stylesheet, an un-sized image causing CLS).

## Common Traps

### Caching Un-Hashed Assets Aggressively

Setting `max-age=31536000` on `app.js` (no hash), so the browser caches it for a year and never fetches the update. Cache immutable assets only when the filename is content-hashed; un-hashed files need short `max-age` with revalidation.

### Long-Caching The HTML Document

Caching `index.html` for a year, so users never see new asset references after a deploy and get a broken mix of new HTML pointing at old assets (or vice versa). Cache HTML with `no-cache` (revalidate) or a short `max-age`.

### A CDN Caching An Authenticated Response Under A Public Key

Serving a personalized page through a CDN keyed only on the URL, missing `private` and `Vary: Authorization`, so one user's data is served to the next. Default authenticated responses to `private`/`no-store`; cache them at shared layers only with an explicitly reviewed per-user key.

### Confusing `no-cache` With `no-store`

Reading `no-cache` as "don't cache" and being surprised it revalidates every time (slower than expected), or using `no-store` when `no-cache` would have allowed cheap 304s. `no-cache` stores-but-revalidates; `no-store` stores nothing. Pick the one matching the freshness/cost tradeoff.

### Omitting `Vary` For A Variant Response

A response that differs by `Accept-Encoding` or `Accept-Language` but does not `Vary` on it, so a compressed or localized variant is served to a request expecting a different one. List every request header that changes the response in `Vary`.

### A CORS Failure That Appears Only In Production

The frontend and API are same-origin in dev (no CORS), but cross-origin in prod, and the preflight was never tested. The first prod deploy fails with a CORS error. Test cross-origin requests and preflights explicitly against a prod-like setup.

### `Allow-Credentials: true` With `Allow-Origin: *` and trusting The Browser Cache For Freshness

The browser rejects this combination (credentials require a specific origin, not the wildcard), and authenticated cross-origin requests fail silently or with a confusing error. Echo the specific requesting origin when credentials are involved.

Relying on the browser cache to reflect the latest version of a resource. The client controls its cache and may serve stale content; for assets, use hashed names so new versions are new resources, and never depend on client-side freshness for correctness.

### An Un-Sized Image Or Late Font Causing Layout Shift and render-Blocking Scripts In The Head

Loading an image without `width`/`height` or a web font that swaps in late, pushing content around after first paint and inflating CLS. Reserve dimensions for async media and use font-display strategies to avoid shift.

Synchronous `<script>` tags in `<head>` that block HTML parsing and delay first paint. Use `defer` (or `async` for independent scripts) so parsing continues while scripts download.

## Self-Check

- [ ] Every resource carries an explicit `Cache-Control` with deliberate directives (`max-age`, `public`/`private`, `no-cache`/`no-store`, `immutable`) chosen by the resource's nature — immutable hashed assets cached aggressively, HTML revalidated, personalized responses `private` or `no-store` — and no resource relies on the stack's default guessing.
- [ ] Static assets use content fingerprinting (hash in filename) with long `max-age` and `immutable`, while the HTML document is cached with `no-cache` or a short `max-age` so a deploy's new references propagate and pull fresh hashed assets — no un-hashed file is cached long-term.
- [ ] For each cached resource, the active mode (freshness/expiration vs validation/revalidation) is known and matches the intent: `max-age` where staleness is tolerable, `no-cache`/`ETag` where freshness is required, and the two are not confused.
- [ ] Personalized or authenticated responses are marked `private` (or `no-store`), `Vary` includes every header that changes the response (`Authorization`, `Cookie`, `Accept-Language`, `Accept-Encoding`), and shared/CDN caching of authenticated content is an explicitly reviewed decision — not a default that risks cross-user leakage.
- [ ] The cache hierarchy (browser, proxy, CDN) was reasoned about: which layers may hold each resource is known, and "stale/wrong data" debugging identifies the holding layer (browser hard-reload vs CDN purge) rather than guessing.
- [ ] Cross-origin requests are configured with correct CORS headers on the server (`Access-Control-Allow-Origin` as a specific origin when credentials are involved, never `*` with `Allow-Credentials: true`), preflight `OPTIONS` is tested explicitly in a prod-like setup, and CORS is treated as the security model working rather than a browser bug.
- [ ] Cookie and authentication mechanics are handled: authenticated cacheability is scoped with `private`/`Vary`, `SameSite`/`Secure`/`HttpOnly` are set deliberately, and credentialled CORS uses `Allow-Credentials: true` with the specific origin and `credentials: 'include'` on the fetch.
- [ ] Performance is grounded in the rendering pipeline and measured web vitals: render-blocking resources are deferred, the LCP element's loading is optimized, CLS sources (un-sized media, late fonts) are eliminated, and long main-thread tasks affecting INP are addressed — optimization was measured against real devices/networks, not assumed.
