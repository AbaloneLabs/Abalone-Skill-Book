---
name: php_performance_and_opcache.md
description: Use when the agent is optimizing PHP application performance, configuring OPcache and JIT, tuning autoloading, optimizing database access and N+1 queries, reducing memory usage, profiling slow requests, or diagnosing bootstrap overhead, cache misses, and memory bloat in PHP and Laravel or Symfony applications.
---

# PHP Performance and OPcache

PHP's traditional request lifecycle (bootstrap, execute, tear down) makes performance a different problem than in long-running runtimes. Every request re-pays the cost of loading and compiling every file unless OPcache holds the compiled bytecode, and the bootstrap cost (autoloader, framework boot, service container compilation) is multiplied by every request. The judgment problem is not "how do I make this function faster" but understanding where time goes across the request lifecycle (compilation, bootstrap, I/O, database, computation) and changing the dominant cost. Most PHP performance issues are I/O (database, external services) and bootstrap (autoloading, framework boot), not CPU, and optimizing CPU in a request whose time is 80% database is wasted effort.

The recurring failure mode is a developer who reaches for micro-optimizations (string concatenation, function call overhead) while the request is dominated by N+1 queries, an unconfigured OPcache, or an un-optimized autoloader. The opposite failure is treating OPcache as a silver bullet: enabling it without tuning memory or revalidate settings, then discovering stale code in production or cache fragmentation under high traffic. Real PHP performance work starts with a profile of the request lifecycle under realistic load, identifies whether the cost is bootstrap, I/O, or computation, and changes the dominant cost first.

## Core Rules

### Enable and tune OPcache for the deployment model

OPcache stores compiled bytecode in shared memory, eliminating recompilation per request. It is essential for production and must be tuned:

- `opcache.memory_consumption`: size the cache to hold your application's compiled code with headroom; monitor `opcache_get_status()` for fullness and restarts.
- `opcache.max_accelerated_files`: set high enough for your file count (use a power of two above the count); too low causes evictions.
- `opcache.validate_timestamps`: in production, set to `0` (no auto-revalidation) and clear the cache on deploy; in development, set to `1` for live code changes.
- `opcache.preload` (since PHP 7.4): precompile and preload a configurable set of files at startup to remove per-request autoload cost; requires careful configuration but can significantly reduce bootstrap.

A misconfigured OPcache (too small, or revalidating in production) either fragments or serves stale code. Monitor it.

### Understand the JIT as a selective accelerator, not a default win

PHP's JIT (since 8.0) compiles hot paths to native code and benefits CPU-bound, long-running work (math loops, image processing). It does little for I/O-bound web requests where time is in database and network waits, and it adds memory and warmup cost. Rules:

- Enable and configure the JIT based on measurement, not by default. The default `tracing` mode is usually a reasonable starting point for long-running work.
- For typical web apps, the JIT's effect is often small; OPcache and I/O optimization dominate.
- In long-lived runtimes (Swoole, RoadRunner), the JIT has more opportunity because warmup amortizes; measure.

### Optimize autoloading to cut bootstrap cost

Every request resolves classes through the autoloader, and a suboptimal autoloader dominates bootstrap time. Rules:

- Run `composer dump-autoload --optimize` (or `--classmap-authoritative`) in production to generate a classmap, eliminating filesystem traversal.
- `--classmap-authoritative` forbids fallback lookups, which is faster but requires the classmap to be complete; use it when you trust the classmap.
- Avoid excessive `require_once` and manual autoloaders; use Composer's PSR-4 with the optimized classmap.
- Preloading (`opcache.preload`) can eliminate autoload resolution for known files entirely.

### Measure and reduce bootstrap overhead

Framework bootstrap (service container compilation, route loading, provider registration) is paid per request. Rules:

- Cache the compiled container and routes (Laravel `route:cache`/`config:cache`/`view:cache`, Symfony cache). In production, these must be cached.
- Minimize eager service providers; defer registration of services not needed on every request.
- Profile the bootstrap separately from the request handling to see its share of total time.
- In long-lived runtimes (Swoole, RoadRunner), bootstrap once and reuse, but be careful of state leakage (see framework-conventions skill).

### Database I/O is usually the dominant cost

For most web requests, database time dwarfs PHP computation. Rules:

- Eliminate N+1 queries by eager-loading relationships (`with` in Eloquent, fetch joins in Doctrine) wherever relationships are iterated.
- Use indexes matched to your queries; an EXPLAIN on slow queries reveals missing indexes and full scans.
- Batch reads and writes; avoid loops that issue one query per item.
- Use connection pooling or persistent connections where the runtime supports it; in classic PHP-FPM, each request opens a connection unless persistent.
- Cache expensive query results with appropriate invalidation, but do not cache stale data that must be fresh.

### Profile the actual request under realistic load

Use a profiler (Xdebug profile, Blackfire, Tideways, `xhprof`) on representative requests, not synthetic benchmarks. Identify:

- **Bootstrap share**: how much of the request is framework boot vs. handling.
- **I/O share**: database, cache, external HTTP calls.
- **Hot functions**: CPU-bound functions called many times.

Optimize the largest share first. A 2x improvement on the dominant cost beats a 10x improvement on a 1% function.

### Memory matters in long-lived and high-concurrency runtimes

In PHP-FPM, memory is freed per request, so leaks are less critical (though worker memory limit matters). In long-lived runtimes (Swoole, RoadRunner), leaks accumulate and crash the process. Rules:

- In FPM, set `memory_limit` per worker appropriately; too low causes fatal errors, too high allows a single request to starve others.
- In long-lived runtimes, explicitly release large buffers and unset references; caches must be bounded.
- Avoid loading entire result sets into memory; stream or paginate large queries.

### Cache at the right layer with the right invalidation

Caching is powerful and dangerous. Rules:

- Cache computed results (views, API responses, query results) with explicit invalidation tied to the underlying data change.
- Prefer cache-aside (read-through, write invalidation) over complex write-through patterns unless consistency demands it.
- Bounded caches with TTLs beat unbounded arrays; an unbounded cache becomes a memory leak.
- Never cache user-specific data under a shared key; key by user where appropriate.

## Common Traps

### OPcache enabled but untuned

Default OPcache settings are often too small for real applications, causing evictions and recompilation. Size memory and file count from `opcache_get_status()`.

### Optimizing CPU in an I/O-bound request

Tuning string operations or function calls in a request that is 80% database is invisible. Profile and fix the dominant cost first.

### N+1 queries hidden by ORM relationship access

Iterating `$post->comments` queries per post. Eager-load and use N+1 detection in tests.

### `validate_timestamps` on in production

Auto-revalidation in production causes stat calls per request and can serve partially-deployed code. Set to `0` and clear on deploy.

### JIT enabled expecting web-app speedup

The JIT helps CPU-bound work; for I/O-bound web requests its effect is small and adds memory cost. Measure before adopting.

### Unbounded caches causing memory growth

A static array or unbounded cache grows across requests (in long-lived runtimes) or within a request (large result sets). Bound caches and stream large data.

### Forgetting route/config/view caching in production

Uncached routes, config, and views re-pay compilation per request. Cache them in production and clear on deploy.

### Persistent connections leaking state

In long-lived runtimes, database connections and services persist; request-scoped state must be reset. A service holding the previous request's user causes cross-user leakage.

## Self-Check

- Is OPcache enabled and tuned (memory, file count, `validate_timestamps=0` in production with cache-clear on deploy), with status monitored for fullness?
- Is the JIT enabled only where measurement shows benefit (CPU-bound/long-running), with the understanding that it does little for I/O-bound web requests?
- Is the autoloader optimized (`composer dump-autoload --optimize`/`--classmap-authoritative`) in production, with preloading configured where beneficial?
- Are framework routes/config/views cached in production, with bootstrap profiled to confirm its share of request time?
- Have N+1 queries been eliminated via eager loading, with EXPLAIN run on slow queries to verify indexes?
- Was the request profiled under realistic load to identify the dominant cost (bootstrap, I/O, computation) before optimizing?
- In long-lived runtimes, are large buffers released, references unset, caches bounded, and request-scoped state reset to avoid leaks and cross-request leakage?
- Are caches bounded with explicit invalidation tied to data changes, keyed per user where user-specific, and never storing stale data that must be fresh?
