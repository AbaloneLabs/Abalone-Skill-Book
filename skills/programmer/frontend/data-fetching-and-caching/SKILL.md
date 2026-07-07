---
name: data_fetching_and_caching.md
description: Use when the agent is fetching data in a frontend app, choosing a data-fetching library or pattern, avoiding request waterfalls, deduplicating concurrent requests, deciding cache invalidation strategy, implementing optimistic updates with rollback, background revalidation, stale-while-revalidate, pagination or infinite scroll, designing loading/error/empty states, or diagnosing duplicated requests and slow serial fetches. Also covers request batching, race conditions from out-of-order responses, dependent versus independent queries, prefetching, mutation and cache update coordination, the failure mode of fetching in leaf components or duplicating the same request, and over- versus under-caching. This skill is about the fetching and caching mechanics and strategy (how, when, what to cache, how to invalidate), and is distinct from rendering-strategies (which decides render mode and where HTML is produced) and from the server-state-as-a-concept framing in component-architecture-and-state.
---

# Data Fetching And Caching

Fetching data in a frontend is not "call the API and set state." It is a concurrency, caching, and consistency problem, and the difficulty is entirely in the parts that are not the happy path. A naive fetch-on-mount works for a demo and then fails in production: the same data is requested five times by five components; a parent renders, then a child renders and fetches, then a grandchild fetches — three serial round trips for data that could have gone in parallel; a user navigates back and refetches everything they just saw; an earlier slow response arrives after a later one and overwrites the correct screen with stale data; a mutation succeeds but the cache is never invalidated, so the UI shows the old value until a refresh. Each of these is a strategy failure, not a bug in the fetch call.

Agents tend to under-invest here because fetching looks solved — `useEffect` plus `fetch` plus `setState` is three lines and it works. The cost appears when the app grows: waterfalls inflate load times, duplicated requests waste bandwidth and hammer the server, caches that are too aggressive show stale data and caches that are too thin refetch on every navigation, and mutations desync from reads because nothing coordinated them. The judgment problem is a set of strategy questions held together: where do fetches originate, how are they parallelized and deduplicated, what is cached and for how long, when is the cache invalidated or revalidated, and how do mutations stay consistent with reads — all while handling loading, error, empty, and race conditions gracefully.

This skill is about the fetching and caching mechanics and strategy. It is distinct from rendering-strategies, which decides the render mode (SSR/SSG/CSR/streaming) and therefore when server-side data is fetched; here the concern is the client-side fetching layer and its cache, assuming the render mode is chosen. It is also distinct from the conceptual framing of "server state is a different kind of state" covered in component-architecture-and-state; here the concern is the operational mechanics — how requests are structured, deduplicated, cached, invalidated, and kept consistent.

## Core Rules

### Fetch At The Boundary, Parallelize Independent Requests, And Avoid Waterfalls

Where a fetch originates determines whether requests run in parallel or serially, and serial fetching is one of the most common and most fixable causes of slow pages. A request waterfall happens when a component renders, fetches, and only then does a child render and fetch — each round trip waiting on the one before it. The discipline:

- **Fetch at route or page boundaries, not in deep leaf components.** The data a view needs should be requested together at the view's entry, so it can be parallelized and so loading/error states belong to the view. A leaf component that fetches its own data is a waterfall waiting to happen.
- **Parallelize independent fetches.** If two pieces of data do not depend on each other, request them at the same time, not one after the other. Two parallel 200ms fetches finish in 200ms; the same two serialized finish in 400ms. Most data a page needs is independent and should be fetched concurrently.
- **Serialize only true dependencies.** If fetch B needs a value from fetch A (a detail endpoint that needs an ID from a list), it must wait — but make the dependency explicit and ensure A is the only thing B waits for. Do not let unrelated fetches get pulled into the dependency chain by accident.

The test: draw the fetch graph for a page. If independent nodes are stacked vertically, you have a waterfall; flatten them. The same data, fetched in parallel at the boundary, is dramatically faster.

### Deduplicate, Cache, And Use A Server-State Library Instead Of Hand-Rolling

The second great cost of naive fetching is duplicated work: two components that need the same data each fetch it independently, wasting a round trip and risking inconsistent results if one response is fresher. And every navigation refetches data the user just saw, because nothing remembered it. The answer is a dedicated server-state layer.

- **Use a data-fetching library (React Query, SWR, Apollo, RTK Query, or equivalent).** These deduplicate concurrent requests for the same key, cache responses, provide loading/error/refetching states, and handle background revalidation — concerns that hand-rolled `useEffect` fetches rebuild badly and inconsistently.
- **Let the cache deduplicate by query key.** Two components requesting the same key share one network request and both receive the result. This is free deduplication the moment you key queries by their inputs.
- **Cache by a stable, serializable key.** The key encodes what the query is (resource + parameters), so the same key always resolves to the same cached data. Keep keys stable and deterministic; a key that includes a new object reference each render defeats the cache.

Hand-rolling fetch logic is the entry point to every fetching bug: no deduplication, no caching, no revalidation, race conditions on rapid refetches. Reach for the library; reserve hand-rolled fetches for one-off, non-cached needs.

### Choose Cache Timing Deliberately: Fresh, Stale, And Revalidation

Caching is not on or off; it is a set of timing decisions about how long data is considered fresh, when it becomes stale, and what happens then. The right strategy depends on how fresh the data must be and how expensive the fetch is.

- **Fresh-while-stale (cache-then-nothing).** Serve cached data and never refetch until invalidated. Good for data that rarely changes (a config, a reference list). Fast and cheap, but can show stale data until an explicit invalidation.
- **Stale-while-revalidate (SWR).** Serve cached data immediately for a fast paint, then refetch in the background and update when the new data arrives. The user sees content instantly and gets fresh data moments later. The default for most read data that changes occasionally.
- **Always-fetch / network-first.** Bypass the cache and fetch on every view. Good for data that must be fresh on every load (a live balance, real-time inventory), at the cost of a loading state on every navigation.
- **Background revalidation on triggers.** Refetch when the window regains focus, on reconnect, or on an interval — so a user returning to a tab sees updated data without manually refreshing. Apply selectively; refetching everything on focus can be wasteful.

The decisive question per query: how stale is acceptable, and what triggers a refresh? Match the strategy to the answer, and set per-query stale times rather than one global policy — a user profile and a live stock price should not share a cache strategy.

### Coordinate Mutations With Cache Invalidation And Optimistic Updates

A mutation changes server data, and the hard part is keeping the read cache consistent with the new reality. Three patterns, chosen by the stakes:

- **Invalidate after mutation.** After a successful mutation, invalidate the affected query keys so they refetch with the new data. Simple and correct; the user sees a brief loading or stale state then the fresh data. The safe default.
- **Update the cache directly (set the new data without refetching).** When you know the resulting state, write it into the cache to avoid a round trip. Faster, but requires the client to correctly predict the server's response — easy to get subtly wrong with server-computed fields.
- **Optimistic update with rollback.** Update the cache immediately as if the mutation succeeded, show the result instantly, and roll back to the previous state if the request fails. Best perceived performance, but it must keep the pre-mutation snapshot and restore it on error — an optimistic update without a rollback path is a bug waiting for the first network failure.

The rule: every mutation must define how it keeps reads consistent. A mutation that succeeds on the server but leaves the cache showing the old value is the classic "I saved it but the UI didn't update" bug. Decide invalidation versus direct update versus optimistic per mutation, and never leave a mutation without a cache-coordination plan.

### Handle Loading, Error, Empty, Race Conditions, And Pagination As First-Class Concerns

Fetching is asynchronous and fallible, and a robust UI handles every state explicitly rather than assuming success — and large lists add their own fetching strategy.

- **Loading / pending.** Show a meaningful loading state (skeleton, spinner, placeholder) sized to avoid layout shift. Distinguish first-load (no data yet) from background-refetch (stale data visible) so the user is not shown a full spinner when they already have content.
- **Error.** Handle fetch failure with a clear, retryable error state — not a silent blank or a crash. Distinguish network errors, 401/403 (auth), and server errors, and respond appropriately (retry, re-auth, report).
- **Empty.** When the fetch succeeds but returns nothing, show a purposeful empty state, not a blank screen that looks broken.
- **Race conditions.** When requests can overlap (rapid param changes, navigation, typing in a search box), an earlier slow response can arrive after a later one and overwrite the correct screen with stale data. Use request cancellation (AbortController), ignore responses whose request is no longer current, or rely on a library that tracks the latest request per key. Any fetch keyed by changing input must guarantee that only the latest response can update state.
- **Pagination and infinite scroll.** Treat each page as its own cached unit keyed by page number or cursor, so back-navigation is instant and pages can be invalidated independently. Infinite scroll loads append-only pages; reserve space for incoming items to avoid layout shift, load ahead of the viewport so the user does not hit a loading wall, and cap the accumulated window so unbounded pages do not exhaust memory.

The failure modes here are fetching an entire list up front (slow, wasteful), refetching already-loaded pages on every scroll, or assuming the fetch succeeds and shipping a blank or crashing UI when it does not.

## Common Traps

### Fetching In Leaf Components Causing Waterfalls

A parent renders, a child renders and fetches, a grandchild renders and fetches — serial round trips for data that could have been requested in parallel at the page boundary. Fetch at the boundary and parallelize independent queries.

### Duplicated Requests And Hand-Rolled Fetch Logic With No Cache

Two components each fetching the same endpoint independently, or `useEffect` + `fetch` + `setState` rebuilt for every query with no caching, no revalidation, no race handling, and no shared loading state. Key queries by their inputs and use a server-state library so concurrent requests deduplicate and results are cached.

### One Global Cache Policy For All Queries

Treating a rarely-changing config and a live balance with the same stale time, so one is always stale and the other shows yesterday's data. Set per-query stale times and revalidation strategies matched to how fresh each resource must be.

### Mutation Without Cache Coordination Or Optimistic Rollback

A successful mutation that does not invalidate or update the affected reads, so the UI keeps showing the pre-mutation value until a manual refresh — or an optimistic update applied with no failure handling, so the first network error leaves the UI showing a state the server never accepted. Every mutation must define how it keeps reads consistent, and every optimistic update must keep a snapshot to restore on failure.

### Race Conditions And Missing Loading/Error States

Rapid param changes where an earlier slow response arrives after a later one and overwrites the correct screen with stale data, or assuming the fetch succeeds and shipping a blank or crashing UI when it does not. Cancel or ignore stale requests for changing keys, and design loading (first-load versus background-refetch), error (retryable, typed), and empty states explicitly for every query.

## Self-Check

- [ ] Data is fetched at route/page boundaries with independent requests parallelized, not in deep leaf components causing serial waterfalls — and true dependencies are the only serialized fetches.
- [ ] Concurrent requests for the same data are deduplicated by a stable, serializable query key, and a server-state library handles caching, deduplication, and revalidation rather than hand-rolled `useEffect` fetch logic.
- [ ] Cache strategy is chosen per query based on how fresh the data must be (fresh-while-stale, stale-while-revalidate, network-first, background revalidation on focus/reconnect/interval), not applied as one global policy.
- [ ] Every mutation defines how it keeps reads consistent — invalidation as the safe default, direct cache update when the result is predictable, or optimistic update with a rollback snapshot restored on failure — and no mutation leaves the cache showing stale data.
- [ ] Requests keyed by changing input are protected against race conditions via request cancellation or ignoring stale responses, so an earlier slow response cannot overwrite a later correct one.
- [ ] Loading (distinguishing first-load from background-refetch), error (retryable and typed by cause), and empty states are designed explicitly for every query rather than treated as afterthoughts.
- [ ] Pagination and infinite scroll treat each page as an independently cached unit keyed by page/cursor, load ahead of the viewport without layout shift, and avoid unbounded memory growth or refetching already-loaded pages.
