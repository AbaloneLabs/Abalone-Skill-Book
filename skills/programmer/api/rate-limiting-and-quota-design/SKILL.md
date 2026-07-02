---
name: rate_limiting_and_quota_design.md
description: Use when the agent is designing rate limiting, choosing limit strategies such as fixed window or token bucket, identifying clients, structuring 429 responses with retry guidance, enforcing fairness, designing hierarchical quotas, or preventing limit bypass and abuse.
---

# Rate Limiting and Quota Design

Rate limiting is the boundary between an API that survives its own success and one that collapses under a single noisy client. But a well-designed limiter is more than a counter that returns `429`. It encodes fairness decisions, cost models, and recovery contracts. A limiter that throttles the wrong dimension punishes well-behaved clients, one that leaks identity lets abusers multiply their allowance, and one that returns a bare `429` with no retry guidance causes synchronized retry storms that amplify the very load it was meant to control.

The judgment problem is deciding what to limit, how to identify the entity being limited, how to communicate limits so clients can cooperate, and how to make limits fair and hard to evade. The agent should not reach for "add a rate limiter" without specifying the strategy, the keying, the response contract, and the bypass surface.

This skill applies whenever you are designing, tuning, or reviewing throttling, quotas, API tier limits, or abuse defenses for any service that serves multiple clients.

## Core Rules

### Decide what dimension you are protecting

Rate limits exist to protect scarce resources, and different resources need different protection:

- **Per-client fairness**: prevent one client from monopolizing capacity. Keyed by API key, user, or IP.
- **Global capacity**: protect a shared downstream (a database, an expensive computation, a third-party API with its own limits). Keyed globally or by resource.
- **Cost-weighted limits**: not all requests are equally expensive. A read of one row and a bulk export of a million rows should not cost the same token. Weight requests by cost when costs vary widely.

Decide which resource is the real bottleneck and key the limiter to the dimension that protects it. A per-IP limit that ignores the API key lets one user behind a NAT exhaust the limit for everyone sharing that IP.

### Choose the limit strategy by its behavioral properties

Each strategy has distinct fairness and smoothness properties:

- **Fixed window** (`count per minute`): simple, but allows bursts at window boundaries (two full bursts in the instant a window rolls over) and is unfair to clients whose requests straddle a boundary.
- **Sliding window** (weighted or log-based): smoother than fixed window, avoids the boundary burst, slightly more memory/compute.
- **Token bucket**: allows short bursts up to the bucket capacity while enforcing an average rate. Best when clients legitimately burst (e.g., a UI loading several widgets at once) but must be bounded on average. The capacity and refill rate are independent knobs.
- **Leaky bucket**: smooths output to a constant rate, useful when downstream is sensitive to bursts.

Weak choice: fixed window with a low limit, causing legitimate bursty clients to be throttled at every window boundary. Strong choice: token bucket with a capacity that permits expected bursts and a refill rate that bounds average load.

### Key the limiter to the right identity

The keying determines fairness. Options, from coarse to fine:

- **IP address**: coarse, shared behind NAT/proxies, spoofable via headers if trusted naively. Use only as a last-resort fallback for unauthenticated traffic.
- **API key / token**: the right key for authenticated clients, but assume keys can be rotated and shared; tie limits to the account, not just the key, if you want per-customer fairness.
- **User/account**: finest grain for fairness, but a single user with many keys can still concentrate load unless you also limit per-key.

Layer the keys: per-key limits prevent one key from dominating, per-account limits prevent one customer from spinning up many keys to evade per-key limits, and global limits protect shared capacity. Hierarchical quotas let you express "1000/min per key, 5000/min per account, 50000/min global."

### Design the 429 response as a cooperation contract

A `429 Too Many Requests` response must tell the client how to cooperate:

- Include `Retry-After` with a concrete wait duration (seconds, or an HTTP-date). Without it, clients guess and often retry immediately, amplifying load.
- Communicate the limit and remaining allowance via headers (e.g., `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`) so well-behaved clients can self-throttle proactively rather than discover limits by hitting them.
- For hierarchical limits, indicate which dimension was exceeded so the client knows whether backing off briefly (per-key) or for longer (per-account) will help.
- Distinguish "you are rate limited, retry later" (`429`) from "the service is shedding load" (`503` with `Retry-After`). Clients treat these differently.

Weak choice: bare `429` with no headers. Strong choice: `429` with `Retry-After`, remaining-balance headers, and a body naming the exceeded limit.

### Make limits fair and explainable

Customers and partners will ask why they were throttled. Limits should be:

- **Documented**: publish the per-tier limits so clients can design around them.
- **Predictable**: a client that stays under its documented allowance should never be surprised by throttling.
- **Explainable**: when throttled, the response should identify the limit and the current usage so the client can self-diagnose.
- **Bounded in penalty**: a single over-limit request should not cascade into a long lockout unless that is an explicit anti-abuse decision.

### Prevent bypass and account for shared identity

Abuse and accidental overload both exploit weak identity:

- Do not trust client-supplied headers (like `X-Forwarded-For`) for keying unless you control the hop that sets them; otherwise an attacker can forge a new IP per request to evade per-IP limits.
- Account for shared identity: many users behind one corporate NAT, or one user behind many keys. If you key only by IP, you punish the NAT group; if you key only by key, one account with many keys evades the limit.
- Consider graceful degradation for legitimate shared-IP cases (a whole office hitting a per-IP limit) rather than hard failure.

### Decide the failure mode when the limiter itself is unavailable

If the rate-limit store (e.g., Redis) goes down, you must choose: fail open (allow all traffic, risking overload of the protected resource) or fail closed (reject all traffic, causing an outage). The right choice depends on the blast radius. For most public APIs, failing open with monitoring and alerting is preferable to a self-inflicted outage, but for resources where overload is catastrophic (e.g., an expensive billing write path), failing closed may be correct. Make this an explicit, documented decision, not an accident.

## Common Traps

### Returning 429 without Retry-After or limit headers

A bare `429` forces clients to guess the backoff. The result is either immediate retries (amplifying load) or excessive caution (degraded UX). The cooperation headers are the point of the response.

### Keying only by IP

IP keying is coarse and shared. Behind NAT or a corporate proxy, one noisy client throttles everyone. Behind rotating proxies, an abuser evades the limit entirely. Use IP only as a fallback for unauthenticated traffic.

### Using fixed windows without considering boundary bursts

Two clients each allowed 100/min can together send 200 requests in the single second a window rolls over, doubling instantaneous load. Fixed windows are simple but bursty; token or sliding windows are smoother.

### Treating all requests as equal cost

Counting requests equally means a cheap read and an expensive export consume the same token. A client that issues a few expensive requests can starve others, or a client that issues many cheap requests can hit the limit while barely loading the system. Weight by cost when costs vary.

### Failing closed on limiter outage without a blast-radius analysis

If the limiter store dies and you reject all traffic, you have turned a partial dependency failure into a full outage. This is sometimes correct but is often an accidental default. Make the failure mode an explicit decision.

### Letting one account evade per-key limits via many keys

Per-key limits alone do not bound a single customer. Without a per-account cap, a determined customer (or abuser) rotates keys to multiply their allowance. Layer account-level limits.

### Synchronized retries after a global limit lifts

If many clients are throttled simultaneously and all retry at the same instant (e.g., exactly when a window resets), they create a thundering herd. Stagger `Retry-After` values with jitter, especially for global or shared limits.

## Self-Check

- Is the limiter keyed to the dimension that actually protects the bottleneck resource (per-key, per-account, global, or a layering of these)?
- Was the strategy (fixed/sliding window, token/leaky bucket) chosen for its behavioral properties, not just simplicity?
- Does every `429` include `Retry-After` and remaining-balance headers so clients can cooperate?
- Are limits documented, predictable, and explainable when a client is throttled?
- Does the design account for shared identity (NAT) and evasion (many keys, forged headers)?
- Are requests weighted by cost where costs vary widely, rather than counted equally?
- Is the failure mode (fail open vs fail closed) when the limiter store is unavailable an explicit, documented decision?
- For hierarchical quotas, does the response indicate which limit dimension was exceeded?
- Are global/shared limits protected against synchronized retry storms via jittered backoff?
- Have you confirmed the limit keying cannot be trivially bypassed by forged client headers?
