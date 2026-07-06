---
name: cache_invalidation_and_strategy.md
description: Use when the agent is deciding whether to cache something, choosing a cache layer (browser, CDN, reverse proxy, application, in-memory, database), setting TTLs, designing cache keys, invalidating or evicting cached data, handling cache stampedes or thundering herd, preventing stale or inconsistent data, caching authenticated or personalized responses, reviewing a caching layer for privacy leaks, or diagnosing bugs that disappear on a cache flush. Also covers write-through versus write-back versus cache-aside, consistency models, negative caching, cache poisoning, the two hard problems (naming and invalidation), and the cost of caches that hide bugs or serve the wrong user's data.
---

# Cache Invalidation And Strategy

A cache is a bet that data will be read more often than it changes, and that a slightly stale copy is acceptable in exchange for speed. The bet pays off handsomely when it is right and causes subtle, expensive failures when it is wrong. The hard part of caching is not adding the cache — it is deciding what to cache, where, for how long, and how to keep it correct when the source changes. "There are only two hard things in computer science: cache invalidation and naming things." The invalidation half is hard because every wrong answer produces a different failure: stale data shown to users, another user's private data served from a shared cache, a thundering herd when a hot key expires, or a bug that vanishes the moment you clear the cache and returns the next day.

Agents tend to under-invest in the strategy because adding a cache is easy and the reward is immediate: the endpoint gets fast, the load drops, the dashboard turns green. The cost is deferred and invisible until a user sees yesterday's price, a logout does not work, or a write appears to have no effect because the read is still served from the old value. The judgment problem is treating each cache as a set of deliberate decisions — what to cache, at which layer, under what key, for how long, invalidated by what, and visible to whom — rather than sprinkling `cache.set` and moving on. A cache you cannot reason about is a cache that will eventually serve the wrong answer and make itself hard to debug.

This skill is about the strategy and correctness of caching. It complements the profiling skill (which covers finding bottlenecks that a cache might solve) and the query-optimization skill (which is often a better first answer than caching). Here the question is: if you cache, how do you keep it correct.

## Core Rules

### Decide Whether To Cache At All, Before Choosing How

Not everything should be cached. A cache adds a correctness surface (staleness, invalidation, key design), an operational surface (memory, eviction, monitoring), and a debugging surface (state that is invisible and diverges from the source). These costs are justified only when the read-to-write ratio and the cost of recomputation make the speed worth it. Before caching, ask:

- **Is the source actually slow?** Profile first. Many "slow" reads are slow for a fixable reason (missing index, N+1 query) that a cache merely masks. Fixing the source is usually better than caching around it.
- **Is the data read more than it changes?** A config value read thousands of times a second and changed monthly is an ideal cache target. A row that is read once and updated on the next request gains nothing from caching and pays the invalidation cost.
- **Can the consumer tolerate staleness?** If the answer must always reflect the latest write, caching is dangerous unless you can invalidate synchronously and reliably. Balance displays and analytics tolerate staleness; authorization decisions and "did my payment succeed" do not.
- **Is the cost of staleness acceptable if invalidation fails?** Every cache is a bet that invalidation will work. If it occasionally does not, what is the worst outcome? Stale product copy is harmless; stale permissions or balances can be a security or financial incident.

The strong choice is often "do not cache, fix the source." The weak choice is caching reflexively to make a number move.

### Choose The Layer By What You Are Trying To Avoid

Caches live at many layers, and each layer avoids a different cost. Choosing the layer is choosing which work to eliminate:

- **Browser / client cache.** Avoids the network round trip entirely. Best for static assets (with content hashing) and for responses safe to serve from the user's own device. Cannot be trusted for anything requiring server-side freshness, because the client controls it.
- **CDN.** Avoids origin load and geographic latency for cacheable, mostly-public content. Strong for static assets and public, anonymous responses; dangerous for personalized or authenticated content unless carefully scoped (see below).
- **Reverse proxy / edge cache.** Avoids application work for responses the proxy can serve. Sits between the network and your code.
- **Application / in-memory cache (Redis, Memcached, process-local).** Avoids database or computation cost. The most flexible layer, and the one where key design and invalidation are entirely your responsibility.
- **Database cache / materialized view.** Avoids repeated expensive queries within the database.

Push the cache as close to the consumer as the data allows. A static asset cached in the browser never hits your infrastructure; the same asset cached only at the database still pays network, TLS, and application cost on every request. But push it only as far as correctness allows — personalized data cannot safely live in a shared CDN edge.

### Design Cache Keys That Are Unique, Stable, And Scoped To Visibility

The cache key decides what a cached entry represents and who can see it. Bad key design is the root cause of most cache correctness bugs: two different things map to the same key, or the same thing maps to different keys, or a key is shared across users who must not see each other's data.

For every key, confirm:

- **It encodes everything that changes the answer.** The key must include every input that affects the cached value: the resource id, the parameters, the variant, and — critically — the identity or tenant that scopes visibility. A key that omits the user id can serve one user's private data to another.
- **It is stable for the same inputs.** Two requests that should return the same answer must produce the same key. Including a timestamp, a request id, or an unordered collection serialized in nondeterministic order defeats the cache.
- **It is scoped to the correct visibility boundary.** Public data can share one key across all users. Per-user data must include the user. Per-tenant data must include the tenant. A common catastrophic bug is a CDN caching a response that contains user-specific data under a public key.
- **It does not collide unintentionally.** Concatenating keys without separators (`user` + `1` + `2` collides with `user` + `12`) or using ambiguous formats produces silent collisions. Use explicit, unambiguous key construction.

Key design is where privacy and correctness intersect. Treat the key as a security-relevant artifact: if it is wrong, the cache becomes a cross-user data leak.

### Pick An Invalidation Strategy That Matches The Cost Of Staleness

Invalidation is how a cache stops being wrong. There are two families, and the choice depends on whether staleness is tolerable and whether the source-of-truth can signal changes:

- **TTL (time-to-live) — accept bounded staleness.** The entry expires after a duration. Simple, robust to invalidation failures, and the right choice when the data changes slowly and consumers tolerate being up to `TTL` behind. The tradeoff is that every value is potentially stale by up to the TTL, and the staleness is unbounded if the TTL is long.
- **Explicit invalidation — evict on change.** When the source changes, the cache entry is deleted or updated. Gives freshness but requires the write path to know which entries to evict — which is exactly the hard problem. Miss an entry and it stays stale indefinitely (worse than TTL, because TTL would eventually self-heal).
- **Versioned / content-keyed — never invalidate, just supersede.** The key includes a version or content hash (e.g., asset filenames with a hash suffix). A new version is a new key; old entries age out by eviction. Eliminates invalidation entirely for data that can be versioned, which is why it is the standard for static assets.

The strongest strategies combine these: a short TTL as a safety net under explicit invalidation, so that a missed invalidation self-heals rather than serving stale data forever. Pure explicit invalidation with no TTL is the most fragile design — one missed event and the cache is permanently wrong until someone notices.

### Prevent Cache Stampedes And Thundering Herd

When a hot key expires or is evicted, and many requests arrive before it is repopulated, all of them miss the cache and all of them recompute the same expensive value simultaneously — overwhelming the source the cache was meant to protect. This is the thundering herd, and it can turn a cache expiry into an outage.

Mitigations:

- **Locking / single-flight / request coalescing.** When a miss occurs, one request recomputes; the others wait for its result rather than all recomputing. The cache layer or application serializes the rebuild.
- **Stale-while-revalidate.** Serve the expired entry immediately while one request refreshes it in the background. Users see slightly stale data instead of a stampede.
- **Early / probabilistic refresh (jittered TTL).** Refresh the entry before it expires, with jitter so many keys do not expire at the same instant. Prevents synchronized expiry across a warm cache after a restart.
- **Backfill / warm-up.** Pre-populate known hot keys after a deploy or cache flush rather than letting the first requests take the miss together.

Any cache that protects an expensive operation under load needs a stampede defense. A cache that "works" until a popular key expires and then collapses has not solved the capacity problem; it has moved it to the expiry moment.

### Treat Personalized And Authenticated Content As A Security Surface

Caching responses that depend on who is asking is where caches become security vulnerabilities. A CDN or reverse proxy that caches a response based only on the URL will serve User A's account page to User B. The defenses are layered:

- **Do not cache authenticated responses by default.** Unless you have explicitly designed the cache key to be per-user and verified it, treat any authenticated response as uncacheable.
- **Vary by everything that changes the answer.** The `Vary` header (or the equivalent cache-key input) must include every request attribute that affects the response: authorization, user id, tenant, locale, feature flags. Omit one and you get cross-user leakage.
- **Mark private responses as private.** Use cache-control directives (`private`, `no-store`) to keep shared caches from storing responses that must not be shared. A missing `private` directive on a personalized response is a leak waiting for a CDN.
- **Separate public from personalized data.** Cache the public parts aggressively; fetch and merge the personalized parts per request. Do not cache a combined response that mixes the two under a public key.

Cross-user cache leakage is a recurring, high-severity bug class. Treat any cache that can see authenticated data as needing an explicit, reviewed decision about what is keyed and what is marked uncacheable.

### Assume The Cache Will Hide Bugs, And Make It Observable

A cache introduces a second copy of the data that can diverge from the source. When it diverges, the symptom is often a bug report that "the data is wrong" or "my change didn't take effect" — and the bug mysteriously disappears when someone clears the cache, only to return. This makes cache-related bugs hard to reproduce and easy to dismiss.

Counter this by making the cache a first-class, observable component:

- **Monitor hit rate, miss rate, and eviction rate.** A collapsing hit rate, a spike in evictions, or a key that never hits all signal problems (bad keys, too-small cache, uncacheable workload).
- **Expose the cache in debugging.** When investigating a "stale data" report, be able to inspect what the cache holds for that key and when it was set. A cache you cannot inspect is a cache you cannot debug.
- **Track staleness where it matters.** For high-stakes data, log or alert when a cached value's age approaches its freshness bound.
- **Have a flush mechanism, but do not rely on it.** Being able to clear the cache is essential for recovery; depending on flushing to hide invalidation bugs is the bug.

A cache that is invisible in observability will produce intermittent, hard-to-trace defects. Design the cache to be inspected, measured, and cleared from the start.

### Account For Eviction And Memory Bounds

Caches have finite memory. When full, they evict entries by a policy (LRU, LFU, TTL, random). Eviction changes the cache's behavior in ways that affect correctness and performance:

- **Eviction is not invalidation.** An evicted entry is simply gone; the next read recomputes it. This is safe but can cause performance cliffs if a working set larger than the cache thrashes (frequent eviction and recompute).
- **Unbounded caches are memory leaks.** A cache that never evicts grows forever and will eventually exhaust memory. Every cache needs an eviction policy and a size bound, or it is a leak by another name.
- **Hot-key eviction.** If a few keys are hot and many are cold, a size policy that evicts by recency may evict the hot keys under a scan of cold keys. Understand whether your workload is access-skewed and choose the policy accordingly.

Size the cache to the working set, not to "as much as will fit." A cache smaller than the working set thrashes and provides neither speed nor correctness benefit; a cache far larger than the working set wastes memory.

## Common Traps

### Caching To Mask A Fixable Source Problem

Adding a cache because a query is slow, when the query is slow because of a missing index or an N+1. The cache hides the problem, adds a staleness and invalidation surface, and the underlying inefficiency remains. Fix the source first; cache only what is inherently expensive.

### CDN Caching A Personalized Response Under A Public Key

Serving an account page or a user-specific dashboard through a CDN that keys only on the URL, so one user's data is served to the next user. Omitting `Vary: Authorization` or a `private` directive on authenticated responses is a cross-user data leak. Default authenticated responses to uncacheable; cache them only with an explicitly reviewed per-user key.

### Key That Omits A Visibility Input

Building a cache key from the resource id and parameters but not the user or tenant, so two users requesting "the same" resource share an entry. The key must encode every input that changes the answer, including identity and scope — treating key construction as security-relevant.

### Pure Explicit Invalidation With No TTL Safety Net

Invalidating cache entries on every write, with no TTL. When one invalidation event is missed (a code path that updates the source without triggering eviction), the entry stays stale forever instead of self-healing. Pair explicit invalidation with a short TTL so missed events recover automatically.

### Same TTL Across All Keys, Expiring Together

Setting every entry to the same TTL, so after a cache restart or a quiet period the entire cache expires at the same instant and every request misses at once, stampeding the source. Add jitter to TTLs and use stale-while-revalidate or single-flight to smooth rebuilds.

### Trusting The Client Cache For Freshness

Relying on the browser cache to reflect the latest version of an asset or response. The client controls its cache and may serve stale content indefinitely. Use content-hashed keys for assets (so new versions are new keys) and never depend on client-side freshness for correctness.

### Negative Caching Without A Bound and cache That Cannot Be Inspected Or Flushed

Caching "not found" or error results to avoid re-querying, but with too long a TTL, so a resource that is created shortly after is reported missing until the cache expires. Negative caching is useful but its TTL must be short, and errors must not be cached as if they were stable "not found" answers.

A cache with no visibility into its contents and no way to clear it, so a stale-data report becomes an untraceable mystery. Make the cache observable (hit/miss/eviction metrics, key inspection) and flushable for recovery.

### Unbounded Cache Treated As Free Memory and invalidation That Does Not Cover All Write Paths

A process-local cache with no size limit or eviction policy, growing until the process runs out of memory and crashes. Every cache needs a bound and an eviction policy; an unbounded cache is a memory leak.

Designing invalidation for the main update path but missing a second path (a background job, an admin tool, a database trigger, a replication lag window) that changes the source without evicting the cache. Invalidation must cover every path that mutates the source, or the cache drifts. Enumerate the write paths before trusting the invalidation.

## Self-Check

- [ ] The decision to cache was deliberate: the source was profiled and found genuinely expensive, the read-to-write ratio justifies caching, and the consumer's tolerance for staleness was confirmed — caching was not a reflexive mask for a fixable inefficiency.
- [ ] The cache layer was chosen by what cost it eliminates, pushed as close to the consumer as correctness allows (browser/CDN for public, application for personalized), and not pushed past the visibility boundary.
- [ ] The cache key encodes every input that changes the answer — including resource id, parameters, variant, and user/tenant identity — is stable for identical inputs, uses unambiguous construction, and is scoped to the correct visibility boundary so no two users share an entry they should not.
- [ ] The invalidation strategy matches the cost of staleness: TTL where bounded staleness is acceptable, explicit invalidation where freshness is required, versioned/content-keyed where possible — and pure explicit invalidation is backed by a TTL safety net so missed events self-heal.
- [ ] A stampede defense (single-flight/locking, stale-while-revalidate, jittered TTL, or warm-up) protects any cache guarding an expensive operation under load, so a hot-key expiry does not collapse into an outage.
- [ ] Authenticated or personalized responses are not cached under public keys; `Vary` includes every attribute that changes the response, private responses are marked `private`/`no-store`, and public and personalized data are cached separately — reviewed as a security surface.
- [ ] The cache is observable: hit/miss/eviction rates are monitored, entries can be inspected for debugging, staleness is tracked where it matters, and a flush mechanism exists for recovery without being relied upon to hide invalidation bugs.
- [ ] The cache has a size bound and an eviction policy appropriate to the access pattern; no unbounded cache exists that would grow until memory is exhausted.
- [ ] Negative caching (of "not found" or errors) uses a short TTL and does not cache transient errors as stable answers.
- [ ] Every write path that mutates the source was enumerated and covered by the invalidation strategy, including background jobs, admin tools, triggers, and replication-lag windows — so the cache cannot drift silently from the source of truth.
