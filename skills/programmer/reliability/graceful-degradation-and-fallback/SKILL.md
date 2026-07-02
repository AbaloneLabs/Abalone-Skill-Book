---
name: graceful_degradation_and_fallback.md
description: Use when the agent is designing fallback behavior for partial failures, choosing between cached or default or degraded responses, implementing feature-flag-based degradation, preserving user experience during dependency outages, or detecting recovery to return to full operation.
---

# Graceful Degradation and Fallback

Systems rarely fail all at once. A recommendation service goes down, a search index falls behind, a feature-flag service times out, and the question is not whether the whole application crashes but whether the user sees a broken page, a stale result, or a sensible default. Graceful degradation is the difference between a partial dependency failure causing a full outage and causing a barely-noticed reduction in capability. Yet degradation is easy to design badly: a fallback that serves wrong data silently, a cached response that never expires and shows hours-old information, or a degraded mode that never recovers because nobody told it the dependency was back.

The judgment problem is deciding, for each non-critical dependency, what the system should do when it fails, how to keep the degraded experience correct and honest, how to gate degradation behind observable controls, and how to detect recovery so the system returns to normal rather than staying degraded forever. The agent should not add a `try/except return default` and call it resilience; degradation is a designed behavior with its own correctness requirements.

This skill applies whenever you are designing how a system behaves when a dependency is slow, unavailable, or returning errors, or reviewing resilience beyond basic retries and circuit breakers.

## Core Rules

### Classify dependencies by criticality before designing fallback

Not every dependency deserves a fallback. Classify each:

- **Critical/path-critical**: the request cannot succeed without it (e.g., the primary database for a write). There is no graceful degradation; the correct behavior is to fail fast and clearly, not to fake a result.
- **Important but substitutable**: the request can succeed with a degraded result (e.g., recommendations replaced by popular items, personalized content replaced by generic). These are the prime candidates for fallback.
- **Non-critical/enhancement**: purely additive features (analytics, social proof, decorative widgets). These should fail silently to a no-op without affecting the core experience.

Decide the classification first. Trying to add fallback to a path-critical dependency produces silently wrong results; failing hard on a non-critical dependency produces avoidable outages.

### Choose the fallback strategy by correctness and freshness requirements

For substitutable dependencies, choose a fallback that is correct enough for the context:

- **Cached last-good value**: serve the most recent successful response, with a bounded staleness. Best when the data changes slowly and stale data is acceptable for a short window. The risk is serving stale data indefinitely if staleness is not bounded or communicated.
- **Static default**: serve a safe default (empty list, generic content, conservative estimate). Best when there is no meaningful cached value. The risk is a default that is subtly wrong (e.g., defaulting a price or a permission to an unsafe value).
- **Computed approximation**: derive a rough answer from available data (e.g., average instead of personalized). Best when an approximation is better than nothing.
- **Feature removal**: omit the failing capability entirely and render the rest of the experience. Best for non-critical enhancements.

Weak choice: a global `catch (e) { return null }` that silently returns null for every failure. Strong choice: a per-dependency fallback whose correctness and staleness bounds are explicit.

### Bound and communicate staleness

Cached fallbacks must have explicit freshness limits:

- Set a maximum staleness (TTL) beyond which the cached value is no longer served, because serving arbitrarily old data can be worse than failing.
- When serving stale data, communicate it where it matters (a `stale` flag, a UI indicator, a response header) so consumers do not treat it as fresh.
- Distinguish "serving cached data because the source is down" from "serving cached data for performance," because the two have different correctness implications.

### Gate degradation behind observable, controllable controls

Degraded modes should not be hidden in code as silent catches. Make them observable and controllable:

- **Feature flags / kill switches**: gate non-critical features behind flags so they can be disabled independently when a dependency is unhealthy, without a deploy. This is often more reliable than automatic degradation.
- **Circuit-breaker-driven degradation**: combine with a circuit breaker so that when a dependency is open (failing repeatedly), the system switches to fallback automatically, and when it closes (recovered), it returns to normal.
- **Observability**: emit metrics and logs for fallback invocations, staleness, and degraded-mode duration, so operators can see how much traffic is degraded and for how long.

### Preserve correctness invariants even when degraded

Degradation must never violate a correctness or safety invariant:

- A fallback for a permissions/authorization check must fail closed (deny), never default to allow.
- A fallback for a price, balance, or inventory must not silently substitute a wrong number that could cause financial harm.
- A fallback that omits data must not cause downstream logic to interpret absence as a meaningful value.

Audit each fallback against the question: "If this fallback is served, could a user do something harmful or see something false?" If yes, the fallback is unsafe.

### Detect recovery and return to normal operation

A common failure is a system that degrades correctly but never recovers because nothing checks whether the dependency is healthy again:

- Pair automatic degradation with health probing (e.g., a circuit breaker's half-open state) so the system periodically tests whether the dependency has recovered.
- When using manual kill switches, define the recovery procedure and ownership so a flag disabled during an incident is re-enabled afterward.
- Alert on prolonged degraded operation: if a service has been serving fallbacks for an hour, that is a signal to investigate, not a new steady state.

### Decide degradation granularity

Degradation can be global, per-tenant, per-region, or per-request:

- **Global**: simplest, but a single dependency issue degrades everyone.
- **Per-tenant/region**: more surgical, isolating impact to affected tenants, but more complex to operate.
- **Per-request**: fine-grained, but requires per-request fallback decisions that can be expensive.

Match granularity to the blast radius you want to control.

## Common Traps

### Silent null/empty fallback for everything

A blanket `return null` or `return []` on any error hides failures and can feed wrong data to downstream logic that treats null as meaningful. Silent fallbacks remove the signal that something is wrong.

### Unbounded staleness

Serving cached data with no TTL means a long outage can serve hours- or days-old data that becomes misleading or wrong. Always bound staleness and prefer failing over serving data older than the bound.

### Defaulting to unsafe values

A fallback that defaults a permission to "allow" or a price to zero to "keep working" creates security and financial harm. Safety-critical and money-critical fallbacks must fail closed.

### Degraded mode that never recovers

Automatic degradation without recovery detection leaves the system stuck in fallback long after the dependency recovers. Pair degradation with health probing.

### Hiding degradation from operators

If fallbacks are silent catches with no metrics, nobody knows the system is degraded until a user complains. Degradation must be observable.

### Treating all dependencies as equally fallback-eligible

Applying the same fallback strategy to a critical write path and a non-critical widget produces either fake successes on critical paths or avoidable failures on trivial ones. Classify first.

### Fallback that is correct in isolation but wrong in combination

Each individual fallback may be sensible, but when several fire simultaneously during a widespread outage, their combination can produce an incoherent or unsafe experience. Consider multi-failure scenarios.

## Self-Check

- Has each dependency been classified (critical / substitutable / non-critical), with fallback applied only where a degraded result is acceptable?
- For each fallback, is the strategy (cached last-good, static default, approximation, feature removal) chosen for correctness in context, not just convenience?
- Are cached fallbacks bounded by a maximum staleness TTL, with staleness communicated to consumers where it matters?
- Are degraded modes gated behind observable controls (feature flags, circuit breakers) rather than silent catches?
- Do fallbacks for safety- or money-critical data fail closed rather than defaulting to unsafe values?
- Is there a mechanism to detect dependency recovery (circuit-breaker half-open, health probes) so the system returns to normal operation?
- Are fallback invocations, staleness, and degraded duration emitted as metrics and alerted on?
- Has the granularity of degradation (global, per-tenant, per-region, per-request) been chosen to control blast radius?
- Have you considered multi-failure scenarios where several fallbacks fire at once?
- Is there a defined recovery procedure and ownership for manually disabled features so they are re-enabled after an incident?
