---
name: cache_security_and_privacy.md
description: Use when the agent is designing or reviewing a cache that stores protected, personalized, tenant-scoped, or user-specific data — session data, user profiles, permission results, search results, rendered pages, or any data whose visibility depends on who is asking — and must prevent cross-user or cross-tenant cache leakage, key collision, cache poisoning, or privacy violations via cached data. Covers cache key design for authorization boundaries, preventing one user's cached response from being served to another, cache poisoning via attacker-influenced keys, privacy and retention of cached personal data, shared-cache (CDN/proxy) risks with authenticated content, and cache as a side channel.
---

# Cache Security And Privacy

A cache is a store that returns a value for a key, and its security rests entirely on whether the key captures everything that determines who is allowed to see that value. This sounds simple and is consistently gotten wrong, because the factors that determine visibility — the user, the tenant, the role, the permission version, the experiment cohort, the region — are often implicit in the request and easy to omit from the key. The result is the cache poisoning and cross-user leakage class of bugs: user A's personalized response is served to user B because the key was based only on the resource id; one tenant's data leaks to another because the key omitted the tenant; a cached permission check returns "allowed" to a user who should be denied because the key did not include the actor. These are not theoretical concerns — cross-tenant cache leakage has been the root cause of real, severe breaches, and the defect is almost always a cache key that was complete for performance and incomplete for authorization.

Agents tend to design cache keys for hit rate (what makes requests match?) rather than for authorization (what makes requests equivalent in visibility?). These are different questions. Two requests for the same resource id may be cache-equivalent for performance and security-distinct for authorization, and optimizing the former without the latter produces leakage. The judgment problem is treating the cache key as a security boundary: every factor that changes what the caller is allowed to see must be in the key, the cache must not store data it is not entitled to persist (privacy, retention), and shared caches (CDNs, proxies) must not hold authenticated or personalized content without explicit per-user segmentation. This skill covers the discipline of caching protected data without enabling cross-user leakage, cache poisoning, or privacy violation.

## Core Rules

### Make The Cache Key An Authorization Boundary

The cache key must include every factor that determines what the caller is allowed to see. If two requests differ on any authorization-relevant dimension, they must have different keys, or one caller will receive the other's cached response.

- **Include the actor (user, service account, API key) in the key for personalized or permission-dependent data.** A response that depends on who is asking must be keyed by who is asking. A key of `resource:42` serves the same response to every user; a key of `resource:42:user:7` segments by user.
- **Include the tenant, organization, or workspace in the key for multi-tenant data.** Cross-tenant leakage via cache is a severe breach class. The tenant must be in the key for any data scoped to a tenant, and the key must be constructed from server-derived tenant identity, never from client-supplied tenant id (which an attacker can change to collide).
- **Include the role, permission version, or feature-flag state when they affect the response.** A cached permission decision or feature-gated response must be invalidated or re-keyed when permissions or flags change; include a permission-version or feature-version token in the key so changes force a miss.
- **Derive authorization-relevant key components from trusted server state.** The user id, tenant, and role in the key must come from the authenticated session, not from request parameters an attacker controls. A client-supplied `user_id` in the key enables deliberate key collision (cache poisoning).

### Do Not Cache Across Authorization Boundaries Even For "Public" Data Without Verification

A common error is caching a response as "public" because the endpoint does not require authentication, when the response actually varies by caller (rate-limit headers, personalized recommendations, A/B cohort) or when the resource is public only under certain conditions.

- **Verify the response is truly caller-independent before caching it as shared.** A response with per-user headers (rate limit remaining, personalized fields) is not shared-cacheable as-is. Strip or segment per-user parts, or key by user.
- **Beware "public" resources that become private.** A document that is public now may be made private later; a cached public copy served after the change is a leak. TTL and invalidation must cover visibility transitions, or public/private state must be in the key.
- **Do not cache authorization decisions positively across users.** A cached "user X may access resource Y" must never be consulted for user Z. Permission caches must be keyed by (actor, resource) at minimum, and invalidated when permissions change.

### Prevent Cache Poisoning Via Attacker-Influenced Keys

If any part of the cache key comes from the request, an attacker who controls that part can collide keys — causing their value to be served to others (poisoning) or others' value to be served to them (leakage). Both are security defects.

- **Do not use raw request input as a key component without normalization and validation.** A URL path, query string, or header used directly in the key is attacker-controlled; path normalization, case folding, and parameter allow-listing prevent collision via variant representations of the same intent.
- **Hash or canonicalize unbounded key components.** Long or unbounded input (URLs, search queries) should be hashed (with a keyed hash if the collision space matters) into a fixed key component, after canonicalization, so equivalent inputs collide only when intended.
- **Do not let one user's write populate a key another user reads, for protected data.** Write-through caches for personalized data must key by the writer's identity; a write by user A populating a key user B reads is a poisoning vector.

### Treat Shared Caches (CDN, Proxy) As A Cross-User Boundary

A CDN or reverse proxy cache is shared across all users, and caching authenticated or personalized content there without segmentation is a direct leakage path. The `Cache-Control` and `Vary` headers are the application's statement of what is safe to share-cache.

- **Mark authenticated or personalized responses with `Cache-Control: private` or `no-store`.** A response that depends on the caller must not be stored in a shared cache. `private` restricts storage to the user's own browser cache; `no-store` prevents caching entirely.
- **Use `Vary` to segment shared caches by request dimensions that change the response.** If a shared-cacheable response varies by `Accept-Encoding` or `Accept-Language`, list them in `Vary` so the cache stores separate entries. Do not rely on this for authorization — authorization requires `private`, not `Vary`, because `Vary` on a user identifier in a shared cache is fragile and often bypassable.
- **Audit CDN caching of dynamic endpoints.** A misconfigured CDN that caches an authenticated page serves one user's account to everyone. Default dynamic endpoints to bypass or `private`, and cache publicly only what is verified caller-independent.

### Respect Privacy And Retention Obligations On Cached Data

Cached data is a copy of data that may be subject to privacy law, retention limits, or deletion obligations. A cache is not exempt from these because it is "temporary."

- **Honor data deletion in the cache.** When a user requests deletion (GDPR erasure) or data is purged, the cached copies must be invalidated or expired, not left to serve until TTL. Design deletion to propagate to caches.
- **Bound retention of cached personal data.** A cache that holds personal data indefinitely (no TTL, no eviction) retains data beyond what minimization allows. Set TTLs consistent with the data's retention obligation.
- **Do not cache data more sensitive than the source permits.** Caching full PII, payment data, or health data in a less-protected cache (e.g., a shared Redis without encryption at rest) can violate obligations the primary store satisfies. Match the cache's protection to the data's sensitivity.
- **Consider the cache as a breach surface.** A cache holding sensitive data is a target; encrypt at rest, restrict access, and segment. The more sensitive the cached data, the higher the protection required.

### Beware Cache Timing And Size As Side Channels

A cache can leak information through timing (a cache hit is faster than a miss) or through resource use observable to another user, enabling side-channel attacks on the existence or state of protected resources.

- **Do not let cache hit/miss timing reveal protected existence.** If a request for a resource that exists is cached and fast, and a request for one that does not exist is uncached and slow, an attacker can enumerate existing resources by timing. Equalize timing or avoid existence-revealing caching for sensitive resources.
- **Be cautious with shared caches that one user's behavior can prime for another.** A cache where user A's access populates an entry user B can detect (via timing or error differences) is a side channel. Segment or avoid for sensitive access patterns.

## Common Traps

### Keying Only By Resource Id For Personalized Data

Caching `resource:42` and serving the same response to every user, leaking one user's personalized or permission-dependent view to others. Include the actor, tenant, role, and permission version in the key.

### Deriving Key Components From Client-Supplied Identity

Using a client-supplied `user_id` or `tenant_id` in the cache key, enabling an attacker to collide keys deliberately (cache poisoning) or read another's data. Derive identity from the authenticated session.

### Caching "Public" Responses That Vary By Caller

Caching an unauthenticated endpoint's response as shared when it includes per-user headers, personalized fields, or cohort-dependent content, leaking per-user data through the shared cache. Verify caller-independence; use `private` or segment.

### Shared Cache (CDN) Storing Authenticated Content

A CDN caching an authenticated page due to missing `Cache-Control: private`/`no-store`, serving one user's account to all visitors. Default dynamic/authenticated responses to bypass or `private`.

### Relying On `Vary` For Authorization

Using `Vary: Cookie` or a user identifier to segment a shared cache for authorization, which is fragile and bypassable. Authorization requires `private` (browser-only) or per-user application caching, not shared-cache `Vary`.

### Not Invalidating Cached Data On Deletion

Leaving cached copies of personal data after a deletion request or purge, violating retention/erasure obligations. Propagate deletion to caches; set TTLs consistent with retention.

### Caching Sensitive Data In A Less-Protected Store

Caching PII, payment, or health data in a shared cache without encryption at rest or access control, violating obligations the primary store meets. Match cache protection to data sensitivity.

### Existence-Revealing Cache Timing

A cache hit/miss timing difference that lets an attacker enumerate which protected resources exist. Equalize timing or avoid existence-revealing caching for sensitive resources.

## Self-Check

- [ ] Cache keys for personalized or permission-dependent data include every authorization-relevant factor (actor, tenant, role, permission version, feature-flag state), and two requests that differ on any such factor produce different keys.
- [ ] Authorization-relevant key components (user id, tenant, role) are derived from the authenticated server-side session, never from client-supplied request parameters that an attacker could change to collide keys.
- [ ] Responses cached as "public" or shared are verified to be genuinely caller-independent (no per-user headers, personalized fields, or cohort-dependent content); per-user or permission-dependent responses use `Cache-Control: private`/`no-store` or per-user application caching.
- [ ] Shared caches (CDN, reverse proxy) do not store authenticated or personalized content; dynamic/authenticated responses default to bypass or `private`, and `Vary` is used only for non-authorization dimensions (encoding, language), not for authorization.
- [ ] Request-derived key components (URL, query, headers) are normalized, validated, and canonicalized or hashed before use, preventing cache poisoning via variant representations or attacker-crafted collisions.
- [ ] Write-through or populated cache entries for protected data are keyed by the writer's identity, so one user's write cannot populate a key another user reads.
- [ ] Cached personal data is subject to the same deletion, retention, and minimization obligations as the source: deletion propagates to caches, TTLs bound retention, and sensitive data is cached only in stores whose protection matches its sensitivity (encryption at rest, access control, segmentation).
- [ ] The cache has been reviewed for side-channel risk: hit/miss timing does not reveal the existence of protected resources, and one user's access cannot prime a cache another user can detect for sensitive access patterns.
