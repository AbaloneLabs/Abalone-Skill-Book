---
name: pwa_and_service_worker_architecture.md
description: Use when the agent is designing a Progressive Web App (PWA), implementing a service worker for offline support and caching, designing cache strategies (cache-first, network-first, stale-while-revalidate), handling service worker lifecycle and versioning, reasoning about background sync and push notifications, or diagnosing PWA issues (stale content from an unupdated cache, a broken update flow leaving users on an old version, a service worker caching sensitive data, offline mutations lost). Also covers the failure mode of a service worker caching the wrong things or failing to update (users stuck on a stale or broken version), caching strategies that serve stale or private data inappropriately, the security surface of a service worker running as a man-in-the-middle on all requests, and the recurring mistake of treating a service worker as a simple cache when it is a programmable network proxy with lifecycle, versioning, and security implications.
---

# PWA And Service Worker Architecture

A Progressive Web App (PWA) uses a service worker — a script running in the background, separate from the page — to enable offline support, caching, background sync, and push notifications, making a web app feel installable and network-resilient. The judgment problem is that a service worker is a programmable network proxy that intercepts every request, and treating it as a simple cache leads to stale content, broken updates, and security issues. A service worker that caches too aggressively leaves users on a stale or broken version, and the update flow (how a new service worker takes over) is a first-class design problem. A caching strategy must match each resource's needs (static assets vs dynamic data vs authenticated content), or it serves stale, wrong, or private data. The service worker runs as a man-in-the-middle on all requests, which is a security surface (it can read and modify requests, and it persists across sessions). The discipline is to design cache strategies per resource, to handle the service worker lifecycle and versioning explicitly, to secure the service worker against misuse, and to handle offline mutations and background tasks safely.

Agents tend to copy a service worker caching recipe and ship it, because the offline demo works. The harm appears as users stuck on a stale version (the old service worker cached the old app and the update flow did not replace it), as broken updates (a new service worker with a bug, cached and served to everyone), as stale or private data served from cache (a cache-first strategy on dynamic or authenticated content), and as lost offline mutations (offline writes not synced, or synced without conflict handling). The judgment is to choose cache strategies deliberately per resource, to design the versioning and update flow, to treat the service worker as a security-relevant component, and to handle offline writeback carefully. A service worker that caches the wrong thing or fails to update can brick the user's experience of the app, and the user has no easy way to recover.

This skill covers cache strategy design, service worker lifecycle and versioning, security considerations, and offline/background features. It complements the http-caching-and-browser-fundamentals skill (HTTP cache headers), the cookies-and-storage skill (client-side storage), and the web-standards-and-interoperability skill (standards compliance). Here the focus is the service worker as a programmable proxy and PWA architecture.

## Core Rules

### Design Cache Strategies Per Resource, Not One Strategy For All

Different resources have different caching needs, and the strategy (how the service worker serves a request) must match each resource:

- **Static, versioned assets (hashed JS/CSS, images): cache-first.** Immutable, hashed assets can be served from cache immediately and cached on first fetch; the cache never goes stale because the hash changes with each version. Cache-first gives fast, offline-capable loads.
- **App shell (the HTML and core assets): cache-first with network update, or stale-while-revalidate.** The shell loads fast from cache and updates in the background; but the HTML is not hashed, so the update flow must handle replacing it (see versioning).
- **Dynamic data (API responses, user content): network-first, falling back to cache.** Fresh data when online, cached data when offline; a network-first strategy with cache fallback serves fresh data normally and degrades gracefully offline.
- **Authenticated or private content: do not cache, or cache with care.** Caching authenticated responses can serve one user's private data to another (if the cache is shared) or stale data after a permission change; cache private content carefully (per-user, with proper cache keys) or not at all.
- **Avoid cache-first for dynamic content.** A cache-first strategy on dynamic content serves stale data silently; reserve cache-first for genuinely immutable resources.

### Handle The Service Worker Lifecycle And Versioning Explicitly

A service worker has a lifecycle (install, activate) and versioning, and the update flow — how a new version takes over — must be designed:

- **A new service worker installs but does not activate immediately (by default).** The new service worker waits until no page is using the old one; this means users can be on the old version for a long time. Decide whether to use skipWaiting (activate immediately) or wait, and how to notify the user of an update.
- **Version the cache and clean up old caches on activate.** Each service worker version should use a versioned cache; on activate, delete old caches to avoid unbounded growth and stale data. Forgetting to clean up old caches leaks storage.
- **Design the update notification flow.** If the new version waits to activate, tell the user an update is available (and let them reload), or force the update if critical; a silent update flow leaves users on a stale version unaware.
- **Handle breaking changes between versions.** A new service worker may expect a new app shell or API; if the old cached shell is served with the new service worker (or vice versa), the mismatch can break the app. Ensure the service worker and the app shell version are consistent.
- **Beware the "stuck on old version" failure.** A service worker that errors during install, or an update flow that never completes, can leave users permanently on an old or broken version; provide a recovery path (a self-unregistering "kill switch" version).

### Treat The Service Worker As A Security-Relevant Component

A service worker intercepts all requests in its scope and persists across sessions, which is a security surface:

- **A service worker is a man-in-the-middle on all scoped requests.** It can read and modify requests and responses; a compromised or buggy service worker can inject content, exfiltrate data, or break the app. Treat the service worker script with the same care as any privileged code.
- **Scope the service worker minimally.** A service worker at the root controls the whole origin; a narrower scope limits its reach. Scope only what the service worker needs to control.
- **Serve the service worker over HTTPS.** Service workers require a secure context (HTTPS) to prevent a man-in-the-middle from injecting the service worker itself; never serve a service worker over plain HTTP (localhost excepted for development).
- **Do not cache sensitive data carelessly.** Cached responses persist in the Cache API and can be read by the service worker (and by a future compromised service worker); avoid caching tokens, sensitive personal data, or authenticated content unless the caching is designed for it (per-user, with expiration and eviction).
- **Validate and sanitize any data the service worker handles.** If the service worker constructs responses or modifies requests, validate inputs to avoid injection; a service worker that reflects untrusted data into responses is an injection vector.

### Handle Offline Mutations And Background Tasks Safely

PWAs support offline writes (background sync) and push notifications, and these have their own correctness concerns:

- **Queue offline mutations and sync when connectivity returns.** Offline writes should be queued (in IndexedDB) and replayed by background sync; design the queue for idempotency (a synced write may retry) and ordering.
- **Handle sync conflicts and failures.** An offline write synced later may conflict with server state (another edit, a deleted record); handle conflicts (merge, reject, prompt) and surface failures to the user rather than silently dropping writes.
- **Do not assume background sync is reliable.** Background sync is best-effort and browser-dependent; it may be delayed or not run. Do not rely on it for critical writes without a fallback (e.g., sync on next foreground).
- **Use push notifications carefully.** Push notifications require user permission and a server-side push service; respect the permission (do not spam), handle notification clicks (navigate to the relevant content), and do not include sensitive data in notifications (they may be visible on a lock screen).

## Common Traps

### One Cache Strategy For All Resources

Using cache-first everywhere, serving stale dynamic data or private authenticated content from cache. Match the strategy to each resource (cache-first for immutable, network-first for dynamic, no-cache for private).

### Users Stuck On A Stale Or Broken Version

A service worker update flow that never completes, or a versioning bug, leaving users permanently on an old or broken app. Design the update flow, version the cache, clean up old caches, and provide a kill-switch recovery path.

### Caching Sensitive Or Authenticated Data

Caching tokens, personal data, or authenticated responses in the shared Cache API, risking cross-user leakage or stale private data. Cache private content per-user with expiration, or not at all.

### Treating The Service Worker As A Simple Cache

Ignoring that the service worker is a programmable, persistent man-in-the-middle with security implications. Treat it as privileged code: scope minimally, serve over HTTPS, sanitize inputs.

### Lost Offline Mutations

Offline writes queued but not synced reliably (background sync is best-effort), or synced without conflict handling. Queue idempotently, handle conflicts, and provide a foreground-sync fallback.

### Forgetting To Clean Up Old Caches

Each version adding a new cache without deleting old ones, leaking storage indefinitely. Version caches and delete old ones on activate.

## Self-Check

- [ ] Cache strategies are chosen per resource: cache-first for immutable hashed assets, stale-while-revalidate or cache-with-update for the app shell, network-first-with-cache-fallback for dynamic data, and no-cache or careful per-user caching for authenticated/private content.
- [ ] The service worker lifecycle and versioning are designed: the cache is versioned, old caches are cleaned up on activate, the update flow (skipWaiting vs wait, user notification) is intentional, breaking changes between versions are handled, and a kill-switch recovery path exists for stuck users.
- [ ] The service worker is treated as security-relevant: scoped minimally, served over HTTPS, sensitive/authenticated data is not cached carelessly, and any data it handles is validated/sanitized.
- [ ] Offline mutations are queued (idempotently, in IndexedDB), synced via background sync with conflict handling and failure surfacing, and a foreground-sync fallback exists since background sync is best-effort.
- [ ] Push notifications (if used) require permission, are not spammy, handle clicks, and exclude sensitive data from the notification payload.
- [ ] The highest-risk cases were verified — stale dynamic data from a cache-first strategy, users stuck on a broken version, cached private data leaking across users, lost offline writes — not only the "it works offline" demo.
