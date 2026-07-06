---
name: cookies_and_storage.md
description: Use when the agent is choosing where to store client-side data (cookies versus localStorage versus sessionStorage versus IndexedDB versus Cache API), setting cookie security attributes (SameSite, Secure, HttpOnly, Domain, Path, cookie prefixes), handling authentication or session state on the client, storing tokens or user preferences or offline data, deciding what is safe to put in JS-readable storage, clearing data on logout, versioning or migrating stored data, reasoning about storage eviction and persistence (navigator.storage.persist), or addressing cookie tracking, third-party cookie deprecation, consent, and GDPR/privacy implications. Also covers the XSS-theft boundary between HttpOnly cookies and localStorage, capacity limits, schema migration in IndexedDB, and storage hygiene. Differentiate from http-caching-and-browser-fundamentals which covers SameSite briefly in the caching context; this skill is the comprehensive client-storage decision.
---

# Cookies And Storage

Every piece of client-side state — a session token, a user preference, an offline cart, a cached API response — has to live somewhere, and the browser offers several fundamentally different stores whose security, lifetime, capacity, and accessibility differ in ways that matter. The judgment problem is not "where do I put this" but "which store matches the *trust level, lifetime, size, and access pattern* of this data — and what attacks or evictions does that choice expose?" A session token in `localStorage` is stolen by any XSS; a tracking cookie without `SameSite` enables cross-site request forgery; a megabyte of data in `localStorage` that should be in IndexedDB blocks the main thread on every read; an offline app that assumed IndexedDB persists loses everything when the browser evicts it under storage pressure.

Agents tend to pick a storage mechanism by familiarity ("I'll just use localStorage") rather than by the data's properties, because a prototype works and the failure modes are invisible until exploited or until the browser is under pressure. The harms are specific and recurring: tokens exfiltrated through an XSS because they sat in a JS-readable store; cross-user or cross-site leakage because a cookie lacked the right `SameSite` or scope; data loss because the app treated browser storage as durable when the browser treats it as best-effort and evictable; privacy violations because a third-party cookie kept tracking users who believed they had logged out. The discipline is to treat each store as a distinct contract with security, lifetime, capacity, and access obligations, and to match the data to the contract deliberately.

This skill is the comprehensive client-storage decision. http-caching-and-browser-fundamentals touches `SameSite` in the caching-and-CORS context; here the focus is the full decision: which mechanism, which security attributes, what privacy and eviction reality, and what hygiene (clearing, versioning, migration) the stored data requires.

## Core Rules

### Match The Data To The Store By Trust Level, Lifetime, Size, And Access Pattern

The five client stores have materially different properties, and choosing by familiarity rather than by the data's nature is the root storage bug:

- **Cookies** are sent to the server automatically on every matching request. Use them for state the *server* must read on every request — primarily authentication/session identifiers and server-readable flags. Their defining feature (auto-transmission) is also their cost (they inflate every request and travel cross-site unless scoped).
- **`localStorage`** is synchronous, string-keyed, durable across tabs and sessions, JS-readable, and small (typically ~5MB). Use it for durable *client-only* data that is small and not sensitive — preferences, UI state, a feature flag. It is synchronous and blocks the main thread, so it is wrong for large or structured data.
- **`sessionStorage`** is `localStorage` scoped to a tab's lifetime. Use it for tab-scoped transient state — a multi-step form in progress, a redirect target. It does not survive tab close and is not shared across tabs.
- **IndexedDB** is asynchronous, transactional, schema-versioned, capable of large structured data (hundreds of MB+) and binary blobs, and JS-readable. Use it for large, structured, or queryable offline data — an offline cache of records, a draft document store. It is more complex but is the only correct choice beyond a few megabytes.
- **Cache API** is designed for storing `Response` objects keyed by `Request`, primarily for service-worker-driven offline and caching. Use it for HTTP responses, not arbitrary application state.

The strong practice: name the data's properties (sensitive? server-needed? large? structured? tab-scoped?) and pick the store whose contract matches. The weak practice is "I'll use localStorage for everything," which produces XSS-vulnerable tokens, blocked main threads, and quota errors.

### Treat Cookie Security Attributes As A Mandatory, Reviewed Set

Cookies carry security as a set of attributes, and omitting any of them is an unmade security decision. For every cookie, set the full set deliberately:

- **`Secure`** — the cookie is only sent over HTTPS. Mandatory for anything sensitive; there is essentially no reason to ship a sensitive cookie without it.
- **`HttpOnly`** — the cookie is not readable by JavaScript (`document.cookie` cannot see it). This is the defense against token theft via XSS. **Session and auth cookies should always be `HttpOnly`** unless there is an extraordinary reason the client must read them.
- **`SameSite`** — controls cross-site transmission. `Strict` sends the cookie only on same-site requests; `Lax` (the modern default) adds top-level GET navigations; `None` sends it on all cross-site requests and **requires `Secure`**. Choose `Strict` or `Lax` by default; use `None` only when you genuinely need cross-site delivery (e.g., embedded third-party context) and always with `Secure`.
- **`Domain` and `Path`** — scope the cookie. Over-broad `Domain` (e.g., `.example.com`) makes the cookie visible to every subdomain, widening the blast radius of any subdomain compromise. Scope cookies as narrowly as the function allows.
- **Cookie prefixes** — `__Host-` prefixes force `Secure`, `Path=/`, no `Domain` (the tightest, host-only scope); `__Secure-` forces `Secure`. Use `__Host-` for session cookies to make the tight scope enforced rather than hoped-for.

The rule: a cookie without an explicit, reviewed attribute set is a vulnerability waiting to be filed. Treat cookie configuration as security review, not boilerplate.

### Respect The XSS-Theft Boundary: HttpOnly Cookies Are Safe, localStorage Is Not

This is the single most consequential storage security distinction. **`localStorage`, `sessionStorage`, IndexedDB, and non-`HttpOnly` cookies are all readable by any JavaScript running on the page.** If the page has an XSS, the attacker's script can read them and exfiltrate them. **An `HttpOnly` cookie cannot be read by JavaScript at all**, so an XSS cannot steal it (though the XSS can still make authenticated requests that carry it — HttpOnly prevents *theft*, not *use*).

The implication for token storage is direct:

- **Strong practice:** keep the session credential in an `HttpOnly`, `Secure`, `SameSite=Lax/Strict` cookie. The client never sees the token; an XSS cannot exfiltrate it. The client identifies "am I logged in" through a separate, non-sensitive flag (e.g., a `logged_in` cookie or a bootstrap payload), not through the token itself.
- **Weak practice:** storing the access/refresh token in `localStorage` (or a non-HttpOnly cookie) so the SPA can read it. This makes the token XSS-stealable, and SPAs are a frequent XSS surface. If you must store a token client-side (e.g., bearer tokens for an API), weigh that the entire XSS surface becomes the token's attack surface.

The discipline: assume XSS will happen; choose the store that limits what XSS can take. Sensitive credentials belong behind `HttpOnly`.

### Never Treat Browser Storage As Durable — Plan For Eviction

The browser's client stores are *best-effort*: when the device is under storage pressure, the browser may evict IndexedDB, `localStorage`, the Cache API, and service-worker registrations to reclaim space. The exception is **persistent storage**, requested via `navigator.storage.persist()`, which asks the browser not to evict and (where granted) changes the eviction priority. The implications:

- **Assume offline data can disappear.** An offline-first app that stores the user's work only in IndexedDB, with no server-side copy, can lose that work on eviction. For anything the user would be harmed by losing, persist a server-side copy or sync on regain.
- **Request persistence for offline-critical apps.** For a genuine offline application, call `navigator.storage.persist()` (and handle the case where the user or browser denies it) so eviction does not silently destroy the offline state.
- **Do not exceed quotas silently.** Each store has quotas; a write that exceeds quota throws. Handle quota errors rather than letting them crash the feature, and clean up stale data.

The strong practice: design as if eviction will happen, sync irreplaceable data to the server, and request persistence when offline durability is a real product requirement. The weak practice is treating IndexedDB as a database you control.

### Version And Migrate Stored Data — It Is A Schema, Not A Throwaway

Application data in `localStorage` and especially IndexedDB outlives the code that wrote it. When you change the shape of stored data (rename a field, restructure a record, change a default), already-stored data from older versions is still present and will be read by new code. Without a versioning and migration strategy, this produces "works on my machine, breaks for existing users" bugs.

- **IndexedDB has explicit schema versioning** (`onupgradeneeded` with a version number). Use it: increment the version on every schema change and write migration logic that transforms existing object stores and records. Never assume a fresh database.
- **`localStorage` has no built-in versioning.** Store a version alongside the data and migrate on read, or namespace keys by version. Detect an old shape and transform it before use.
- **Always clear auth/session data on logout** and on sign-in of a different user, so one user's data is not served to another on a shared device. Stale session data on a shared device is a real cross-user leak.

The rule: stored data has a schema; schemas change; plan the migration or accept the corruption.

### Address Privacy And Tracking Implications Deliberately

Client storage is also a privacy surface, and the choices have legal and trust consequences:

- **Third-party cookies are being deprecated.** Browsers are phasing out third-party (cross-site) cookies; designs that depend on them for tracking or embedded functionality must move to alternatives (Storage Access API, first-party data, server-side association). Do not build new functionality on third-party cookies.
- **Storage can be a fingerprinting and tracking vector.** A unique identifier in `localStorage` persists across sessions and can track users who clear cookies but not site data. For privacy-sensitive contexts, avoid planting durable cross-session identifiers; prefer session-scoped or server-held identifiers.
- **Consent and regulation apply.** Under GDPR and similar regimes, storing identifiers or personal data may require user consent and must be clearable. Provide a real "clear my data" path and honor consent choices before writing non-essential storage.
- **Scope data to the user and clear on logout.** Storage that survives logout can leak one user's data to the next on shared devices.

The discipline: ask, for each piece of stored data, whether it identifies or tracks the user, whether consent is required, and whether it is cleared when it should be. Privacy is a storage decision, not just a policy page.

## Common Traps

### Storing The Auth Token In localStorage "Because The SPA Needs It"

Putting the access or refresh token in `localStorage` so client JS can attach it to requests, making the entire XSS surface a token-theft surface. Prefer `HttpOnly` cookies so the client never handles the credential; if a bearer token is unavoidable, accept and document the elevated XSS risk.

### Setting A Session Cookie Without `HttpOnly` Or `Secure`

A session cookie readable by `document.cookie` and transmittable over HTTP, trivially stealable by the first XSS or the first mixed-content downgrade. Session cookies are `HttpOnly`, `Secure`, `SameSite=Lax/Strict`, and ideally `__Host-` prefixed — always.

### Using `SameSite=None` Without `Secure`

The browser rejects `SameSite=None` cookies that lack `Secure`, so the cookie is silently dropped and cross-site functionality breaks with no error. `None` requires `Secure`; if you do not need cross-site delivery, prefer `Lax` or `Strict`.

### Over-Scoping Cookie `Domain` To The Apex

Setting `Domain=.example.com` so a cookie is shared across subdomains, unintentionally exposing it to every subdomain (including less-trusted ones) and widening any compromise. Scope cookies to the narrowest host that serves the function; use `__Host-` to enforce host-only scope.

### Treating IndexedDB As A Durable Database

Storing the user's only copy of offline work in IndexedDB with no server sync, then losing it when the browser evicts under storage pressure. Browser storage is best-effort; sync irreplaceable data to the server and request persistence for offline-critical apps.

### Using localStorage For Large Or Structured Data

Stuffing megabytes of JSON into `localStorage`, blocking the main thread on every synchronous read/write and hitting quota errors. Large or structured data belongs in IndexedDB (asynchronous, transactional, higher capacity); `localStorage` is for small client-only preferences.

### Changing Stored Data Shape Without Migration and leaving One User's Data For The Next On A Shared Device

Renaming a field in the code without versioning the stored data, so existing users read old-shape records and break. IndexedDB migrations belong in `onupgradeneeded`; `localStorage` needs a stored version and transform-on-read. Assume old data exists.

Not clearing storage on logout, so user B sees user A's cached data or session remnants. Clear auth/session data and any per-user cached state on logout and on sign-in of a different user.

### Planting A Durable Tracking Identifier Without Considering Privacy and assuming Quota Writes Always Succeed

Writing a permanent unique ID to `localStorage` for analytics, creating a tracking vector that survives cookie clears and may require consent under privacy law. Prefer session-scoped or server-held identifiers, and honor consent before writing non-essential storage.

Calling `localStorage.setItem` or an IndexedDB put without handling the quota-exceeded error, so the feature fails silently when storage is full. Catch quota errors, surface them, and clean up stale data.

## Self-Check

- [ ] Each piece of client-side data is stored in the mechanism whose contract matches its properties — server-readable auth state in cookies, small durable client-only data in `localStorage`, tab-scoped transient state in `sessionStorage`, large/structured/queryable data in IndexedDB, HTTP responses in the Cache API — and nothing sensitive or large is in `localStorage` merely by familiarity.
- [ ] Every cookie carries a deliberately reviewed attribute set — `Secure`, `HttpOnly` for session/auth cookies, `SameSite` (`Lax`/`Strict` by default, `None` only with `Secure` and a real cross-site need), narrowly scoped `Domain`/`Path`, and `__Host-`/`__Secure-` prefixes where the tightest scope applies — with no cookie missing a security attribute by oversight.
- [ ] Session and authentication credentials are stored in `HttpOnly` cookies (not `localStorage` or JS-readable cookies) so an XSS cannot exfiltrate them, and where a bearer token must be client-held the elevated XSS risk is documented and accepted.
- [ ] Browser storage is treated as best-effort: irreplaceable offline data is synced to the server, `navigator.storage.persist()` is requested for offline-critical apps (with denial handled), and quota-exceeded errors are caught rather than crashing the feature.
- [ ] Stored data has a versioning and migration strategy — IndexedDB schema changes go through `onupgradeneeded` with incrementing versions and migration logic, `localStorage` carries a version and transforms on read — so existing users' old-shape data does not break new code.
- [ ] Auth, session, and per-user cached data are cleared on logout and on sign-in of a different user, so one user's data is not served to another on a shared device.
- [ ] Privacy and tracking implications are addressed: no new dependence on deprecated third-party cookies, durable cross-session identifiers are avoided or consent-gated where required, and a real "clear my data" path exists and honors consent choices.
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
